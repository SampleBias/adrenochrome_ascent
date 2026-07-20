//! Raycast interaction for doors / terminals / valves / elevators (TODO-009).

use bevy::prelude::*;

use adrenochrome_content::InteractAction;
use adrenochrome_engine::{cast_ray, MapGrid, RayCamera};

use crate::combat::CombatTarget;
use crate::enemy::{Enemy, FloorAlarm, ScientistFight};
use crate::game::{EndingKind, GameState};
use crate::hazard::TimedValveState;
use crate::puzzle::{apply_effects, DnaSequencer, PuzzleRegistry};
use crate::audio::PaAnnouncement;
use crate::ui::TerminalSession;

/// Max interaction reach in map cells.
pub const INTERACT_RANGE: f32 = 2.2;
/// How far off the view ray a target may sit.
pub const INTERACT_LATERAL: f32 = 0.55;

/// World interactable driven by floor RON data.
#[derive(Component, Debug, Clone)]
pub struct Interactable {
    pub prompt: String,
    pub require: Option<String>,
    pub action: InteractAction,
}

/// HUD prompt for the currently aimed interactable.
#[derive(Resource, Debug, Clone, Default)]
pub struct InteractionPrompt {
    pub text: Option<String>,
    pub blocked: bool,
}

/// Fired when the player presses interact.
#[derive(Message, Debug, Clone)]
pub struct InteractAttempt;

#[derive(Component, Debug, Clone, Copy)]
pub struct PromptUi;

/// Aim + prompt update (every frame in-game).
pub fn update_interaction_prompt(
    camera: Res<RayCamera>,
    map: Res<MapGrid>,
    registry: Res<PuzzleRegistry>,
    mut prompt: ResMut<InteractionPrompt>,
    query: Query<(&Interactable, &Transform)>,
) {
    let wall_dist = cast_ray(&map, camera.pos, camera.dir).0;
    let mut best: Option<(f32, String, bool)> = None;

    for (interact, tf) in &query {
        let pos = Vec2::new(tf.translation.x, tf.translation.y);
        let Some(depth) = aim_depth(camera.pos, camera.dir, pos) else {
            continue;
        };
        if depth > INTERACT_RANGE || depth > wall_dist + 0.15 {
            continue;
        }
        let ok = interact
            .require
            .as_deref()
            .map(|c| registry.evaluate(c))
            .unwrap_or(true);
        let text = if ok {
            format!("[E] {}", interact.prompt)
        } else {
            format!("[locked] {}", interact.prompt)
        };
        if best.as_ref().map(|(d, _, _)| depth < *d).unwrap_or(true) {
            best = Some((depth, text, !ok));
        }
    }

    if let Some((_, text, blocked)) = best {
        prompt.text = Some(text);
        prompt.blocked = blocked;
    } else {
        prompt.text = None;
        prompt.blocked = false;
    }
}

/// Press E to interact with the aimed target.
pub fn try_interact(
    keys: Res<ButtonInput<KeyCode>>,
    camera: Res<RayCamera>,
    mut map: ResMut<MapGrid>,
    mut registry: ResMut<PuzzleRegistry>,
    mut valves: ResMut<TimedValveState>,
    mut dna: ResMut<DnaSequencer>,
    mut scientist_fight: ResMut<ScientistFight>,
    mut ending: ResMut<EndingKind>,
    mut alarm: ResMut<FloorAlarm>,
    mut terminal: ResMut<TerminalSession>,
    mut pa: ResMut<PaAnnouncement>,
    mut next_state: ResMut<NextState<GameState>>,
    mut attempts: MessageWriter<InteractAttempt>,
    query: Query<(Entity, &Interactable, &Transform)>,
    mut bosses: Query<(&Enemy, &mut CombatTarget)>,
) {
    if !keys.just_pressed(KeyCode::KeyE) {
        return;
    }
    attempts.write(InteractAttempt);

    let wall_dist = cast_ray(&map, camera.pos, camera.dir).0;
    let mut best: Option<(f32, Entity)> = None;
    for (entity, interact, tf) in &query {
        let pos = Vec2::new(tf.translation.x, tf.translation.y);
        let Some(depth) = aim_depth(camera.pos, camera.dir, pos) else {
            continue;
        };
        if depth > INTERACT_RANGE || depth > wall_dist + 0.15 {
            continue;
        }
        let ok = interact
            .require
            .as_deref()
            .map(|c| registry.evaluate(c))
            .unwrap_or(true);
        if !ok {
            continue;
        }
        if best.as_ref().map(|(d, _)| depth < *d).unwrap_or(true) {
            best = Some((depth, entity));
        }
    }

    let Some((_, entity)) = best else {
        return;
    };
    let Ok((_, interact, _)) = query.get(entity) else {
        return;
    };
    apply_action(
        &interact.action,
        &mut registry,
        &mut valves,
        &mut dna,
        &mut scientist_fight,
        &mut ending,
        &mut alarm,
        &mut terminal,
        &mut pa,
        &mut next_state,
        &mut map,
        &mut bosses,
    );
}

fn apply_action(
    action: &InteractAction,
    registry: &mut PuzzleRegistry,
    valves: &mut TimedValveState,
    dna: &mut DnaSequencer,
    scientist_fight: &mut ScientistFight,
    ending: &mut EndingKind,
    alarm: &mut FloorAlarm,
    terminal: &mut TerminalSession,
    pa: &mut PaAnnouncement,
    next_state: &mut NextState<GameState>,
    map: &mut MapGrid,
    bosses: &mut Query<(&Enemy, &mut CombatTarget)>,
) {
    match action {
        InteractAction::SetFlag(flag) => {
            registry.set(flag.clone(), true);
            info!("Flag set: {flag}");
        }
        InteractAction::OpenDoor { cell, flag } => {
            open_door_cells(map, cell.0, cell.1);
            if let Some(flag) = flag {
                registry.set(flag.clone(), true);
            }
            info!("Opened door at {},{}", cell.0, cell.1);
        }
        InteractAction::CallElevator => {
            next_state.set(GameState::ElevatorTransition);
        }
        InteractAction::ReleaseSubjects => {
            *ending = EndingKind::Released;
            registry.set("subjects_released", true);
            registry.add_counter("moral_score", 2);
            info!("Moral flag: subjects_released");
        }
        InteractAction::MoralChoice {
            flag,
            release_subjects,
            score,
        } => {
            registry.set(flag.clone(), true);
            registry.add_counter("moral_score", *score);
            if *release_subjects {
                *ending = EndingKind::Released;
                registry.set("subjects_released", true);
            }
            info!(
                "MoralChoice({flag}) score+{score} release={release_subjects}"
            );
        }
        InteractAction::TriggerAlarm => {
            alarm.raise(45.0);
            registry.set("floor_alarm", true);
        }
        InteractAction::TimedValve { flag, window_secs } => {
            crate::hazard::arm_timed_valve_from_interact(
                flag,
                *window_secs,
                valves,
                registry,
            );
        }
        InteractAction::CollectLimb { amount } => {
            registry.add_counter("collected_limb", *amount);
            info!(
                "Collected limb ×{amount} → {}",
                registry.counter("collected_limb")
            );
        }
        InteractAction::StartPuzzle(id) => {
            let seq = match id.as_str() {
                "scientist_dna_2" => "TGCA",
                _ => "ACGT",
            };
            dna.start(id.clone(), seq);
        }
        InteractAction::OpenTerminal {
            title,
            body,
            set_flag,
        } => {
            terminal.open(title.clone(), body.clone(), set_flag.clone());
            info!("Opened terminal: {title}");
        }
        InteractAction::AnnouncePa { text, duration } => {
            pa.announce(text.clone(), *duration);
            info!("PA: {text}");
        }
        InteractAction::RunEffects { require, effects } => {
            let ok = require
                .as_deref()
                .map(|c| registry.evaluate(c))
                .unwrap_or(true);
            if !ok {
                info!("RunEffects blocked by condition");
                return;
            }
            apply_effects(effects, registry, map, scientist_fight, alarm, bosses);
        }
    }
}

/// Walk-over biometric limb pickups.
#[derive(Component, Debug, Clone, Copy)]
pub struct LimbPickup {
    pub amount: i32,
}

pub fn collect_limbs(
    mut commands: Commands,
    mut registry: ResMut<PuzzleRegistry>,
    player: Query<&crate::player::PlayerMotor, With<crate::player::Player>>,
    limbs: Query<(Entity, &LimbPickup, &Transform)>,
) {
    let Ok(motor) = player.single() else {
        return;
    };
    for (entity, limb, tf) in &limbs {
        let pos = Vec2::new(tf.translation.x, tf.translation.y);
        if motor.pos.distance(pos) > 0.7 {
            continue;
        }
        registry.add_counter("collected_limb", limb.amount);
        info!(
            "Picked up limb → {}",
            registry.counter("collected_limb")
        );
        commands.entity(entity).despawn();
    }
}

fn open_door_cells(map: &mut MapGrid, x: i32, y: i32) {
    for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
        clear_cell(map, x + dx, y + dy);
    }
}

fn clear_cell(map: &mut MapGrid, x: i32, y: i32) {
    if x < 0 || y < 0 {
        return;
    }
    let tex = map.get(x as isize, y as isize);
    if tex != 0 {
        map.set(x as usize, y as usize, 0);
    }
}

fn aim_depth(origin: Vec2, dir: Vec2, target: Vec2) -> Option<f32> {
    let rel = target - origin;
    let depth = rel.dot(dir);
    if depth <= 0.05 {
        return None;
    }
    let lateral = (rel - dir * depth).length();
    if lateral > INTERACT_LATERAL {
        return None;
    }
    Some(depth)
}

/// Sync prompt text into a small bottom HUD label.
pub fn sync_prompt_ui(
    prompt: Res<InteractionPrompt>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    existing: Query<Entity, With<PromptUi>>,
) {
    if !prompt.is_changed() {
        return;
    }
    for e in &existing {
        commands.entity(e).despawn();
    }
    let Some(text) = prompt.text.clone() else {
        return;
    };
    let color = if prompt.blocked {
        Color::srgb(0.75, 0.35, 0.35)
    } else {
        Color::srgb(0.85, 0.85, 0.7)
    };
    commands
        .spawn((
            PromptUi,
            Name::new("InteractionPromptUi"),
            Node {
                position_type: PositionType::Absolute,
                bottom: px(28),
                left: px(0),
                width: percent(100),
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font: FontSource::Handle(asset_server.load("fonts/FiraSans-Bold.ttf")),
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(color),
            ));
        });
}
