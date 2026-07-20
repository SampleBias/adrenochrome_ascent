//! Enter/exit handlers and elevator timer for [`GameState`](super::states::GameState).

use std::time::Duration;

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use adrenochrome_engine::{ActivePalette, RayCamera};

use crate::enemy::{FactionRegistry, FloorAlarm};
use crate::player::{Armor, Health, Inventory, MutationPerks, Player, PlayerMotor, WeaponLoadout};
use crate::puzzle::PuzzleRegistry;
use crate::save::{queue_load_from_slot, ActiveSaveSlot, PendingLoad};

use super::floor::{CurrentFloor, EndingKind};
use super::states::GameState;

/// How long an elevator ride lasts before the next floor (or ending).
pub const ELEVATOR_DURATION: Duration = Duration::from_millis(1600);

/// Counts down during [`GameState::ElevatorTransition`].
#[derive(Resource, Debug, Clone)]
pub struct ElevatorTimer {
    pub timer: Timer,
}

impl Default for ElevatorTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(ELEVATOR_DURATION, TimerMode::Once),
        }
    }
}

/// Keyboard cursor for main-menu rows (TODO-040).
#[derive(Resource, Debug, Clone, Copy)]
pub struct MenuCursor {
    pub index: usize,
    pub load_slot: u8,
}

impl Default for MenuCursor {
    fn default() -> Self {
        Self {
            index: 0,
            load_slot: 1,
        }
    }
}

impl MenuCursor {
    pub const MAIN_ITEMS: usize = 5; // New, Load, Options, Credits, Quit
}

/// Where Options returns when dismissed.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptionsReturn {
    #[default]
    MainMenu,
    InGame,
}

/// When true, the next InGame enter skips floor reload (ESC options resume).
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct SoftInGameResume(pub bool);

/// Apply the floor cluster palette when entering a playable floor.
pub fn apply_floor_palette(floor: Res<CurrentFloor>, mut active: ResMut<ActivePalette>) {
    let next = floor.palette();
    if active.palette != next {
        active.palette = next;
        info!(
            "Floor {} ({}) → palette {:?}",
            floor.number,
            floor.cluster_name(),
            next
        );
    }
}

/// Ensure a player exists when entering the game.
pub fn enter_in_game_spawn_player(
    mut commands: Commands,
    camera: Res<RayCamera>,
    players: Query<Entity, With<Player>>,
) {
    if players.is_empty() {
        commands.spawn((
            Name::new("Player"),
            Player,
            PlayerMotor::from_camera(&camera),
            Health::default(),
            Armor::default(),
            Inventory::default(),
            WeaponLoadout::default(),
        ));
    }
}

/// Release the cursor when leaving gameplay for menus / endings / elevator.
pub fn release_mouse(mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut options) = cursor.single_mut() {
        options.visible = true;
        options.grab_mode = CursorGrabMode::None;
    }
}

/// Start the elevator countdown.
pub fn enter_elevator(mut elevator: ResMut<ElevatorTimer>) {
    elevator.timer.reset();
    info!("Elevator transition started (floor ride)");
}

/// Tick the elevator; advance floor or go to ending when the timer finishes.
pub fn tick_elevator(
    time: Res<Time>,
    mut elevator: ResMut<ElevatorTimer>,
    mut floor: ResMut<CurrentFloor>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    elevator.timer.tick(time.delta());
    if !elevator.timer.just_finished() {
        return;
    }
    complete_elevator(&mut floor, &mut next_state);
}

fn complete_elevator(floor: &mut CurrentFloor, next_state: &mut NextState<GameState>) {
    if floor.advance() {
        info!("Arrived at floor {}", floor.number);
        next_state.set(GameState::InGame);
    } else {
        info!("Surface reached — entering Ending");
        next_state.set(GameState::Ending);
    }
}

/// Reset run state when returning to the main menu.
pub fn enter_main_menu_reset(
    mut floor: ResMut<CurrentFloor>,
    mut ending: ResMut<EndingKind>,
    mut registry: ResMut<PuzzleRegistry>,
    mut perks: ResMut<MutationPerks>,
    mut factions: ResMut<FactionRegistry>,
    mut alarm: ResMut<FloorAlarm>,
    mut cursor: ResMut<MenuCursor>,
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    floor.reset();
    *ending = EndingKind::default();
    registry.clear();
    perks.clear();
    factions.clear();
    alarm.clear();
    *cursor = MenuCursor::default();
    for entity in &players {
        commands.entity(entity).despawn();
    }
}

/// Watch for player death → GameOver (TODO-038).
pub fn watch_player_death(
    player: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(health) = player.single() else {
        return;
    };
    if health.is_dead() {
        next_state.set(GameState::GameOver);
    }
}

/// Debug-only elevator skip (TODO-038: gated for ship).
pub fn request_elevator(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    #[cfg(debug_assertions)]
    if keys.just_pressed(KeyCode::KeyL) {
        next_state.set(GameState::ElevatorTransition);
    }
    #[cfg(not(debug_assertions))]
    let _ = (keys, next_state);
}

/// Menu / ending / elevator / options / credits / game-over keyboard flow.
pub fn flow_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut floor: ResMut<CurrentFloor>,
    mut cursor: ResMut<MenuCursor>,
    mut slot: ResMut<ActiveSaveSlot>,
    mut pending: ResMut<PendingLoad>,
    mut options_return: ResMut<OptionsReturn>,
    mut soft_resume: ResMut<SoftInGameResume>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
    mut settings: ResMut<super::GameSettings>,
) {
    match state.get() {
        GameState::MainMenu => {
            if keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS) {
                cursor.index = (cursor.index + 1) % MenuCursor::MAIN_ITEMS;
            }
            if keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW) {
                cursor.index = cursor
                    .index
                    .checked_sub(1)
                    .unwrap_or(MenuCursor::MAIN_ITEMS - 1);
            }
            if keys.just_pressed(KeyCode::ArrowLeft) || keys.just_pressed(KeyCode::KeyA) {
                if cursor.index == 1 {
                    cursor.load_slot = if cursor.load_slot <= 1 {
                        10
                    } else {
                        cursor.load_slot - 1
                    };
                }
            }
            if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::KeyD) {
                if cursor.index == 1 {
                    cursor.load_slot = if cursor.load_slot >= 10 {
                        1
                    } else {
                        cursor.load_slot + 1
                    };
                }
            }
            if keys.just_pressed(KeyCode::Enter) {
                match cursor.index {
                    0 => {
                        pending.save = None;
                        next_state.set(GameState::InGame);
                    }
                    1 => {
                        slot.slot = cursor.load_slot;
                        if queue_load_from_slot(*slot, &mut pending) {
                            floor.number = pending
                                .save
                                .as_ref()
                                .map(|s| s.floor)
                                .unwrap_or(1)
                                .clamp(1, CurrentFloor::MAX);
                            next_state.set(GameState::InGame);
                        }
                    }
                    2 => {
                        *options_return = OptionsReturn::MainMenu;
                        next_state.set(GameState::Options);
                    }
                    3 => next_state.set(GameState::Credits),
                    _ => {
                        exit.write(AppExit::Success);
                    }
                }
            }
            if keys.just_pressed(KeyCode::Escape) {
                exit.write(AppExit::Success);
            }
        }
        GameState::Options => {
            if keys.just_pressed(KeyCode::KeyM) {
                settings.music_volume = ((settings.music_volume * 10.0).round() as i32 + 1).rem_euclid(11)
                    as f32
                    / 10.0;
            }
            if keys.just_pressed(KeyCode::KeyN) {
                settings.sfx_volume = ((settings.sfx_volume * 10.0).round() as i32 + 1).rem_euclid(11)
                    as f32
                    / 10.0;
            }
            if keys.just_pressed(KeyCode::KeyC) {
                settings.crt_enabled = !settings.crt_enabled;
            }
            if keys.just_pressed(KeyCode::KeyV) {
                settings.dither_enabled = !settings.dither_enabled;
            }
            if keys.just_pressed(KeyCode::KeyF) {
                settings.fullscreen = !settings.fullscreen;
            }
            if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::Enter) {
                match *options_return {
                    OptionsReturn::InGame => {
                        soft_resume.0 = true;
                        next_state.set(GameState::InGame);
                    }
                    OptionsReturn::MainMenu => next_state.set(GameState::MainMenu),
                }
            }
        }
        GameState::Credits => {
            if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::Enter) {
                next_state.set(GameState::MainMenu);
            }
        }
        GameState::GameOver => {
            if keys.just_pressed(KeyCode::Enter) {
                if queue_load_from_slot(*slot, &mut pending) {
                    floor.number = pending
                        .save
                        .as_ref()
                        .map(|s| s.floor)
                        .unwrap_or(floor.number)
                        .clamp(1, CurrentFloor::MAX);
                }
                next_state.set(GameState::InGame);
            }
            if keys.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::MainMenu);
            }
        }
        GameState::Ending => {
            if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::MainMenu);
            }
        }
        GameState::ElevatorTransition => {
            if keys.just_pressed(KeyCode::Enter) {
                complete_elevator(&mut floor, &mut next_state);
            }
        }
        GameState::InGame => {
            if keys.just_pressed(KeyCode::Escape) {
                *options_return = OptionsReturn::InGame;
                next_state.set(GameState::Options);
            }
        }
    }
}
