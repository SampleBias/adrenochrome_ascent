//! Enter/exit handlers and elevator timer for [`GameState`](super::states::GameState).

use std::time::Duration;

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use adrenochrome_engine::{ActivePalette, RayCamera};

use crate::enemy::{FactionRegistry, FloorAlarm};
use crate::player::{Armor, Health, Inventory, MutationPerks, Player, PlayerMotor, WeaponLoadout};
use crate::puzzle::PuzzleRegistry;

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
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    floor.reset();
    *ending = EndingKind::default();
    registry.clear();
    perks.clear();
    factions.clear();
    alarm.clear();
    for entity in &players {
        commands.entity(entity).despawn();
    }
}

/// Dev/playtest: call the elevator from in-game (placeholder until interactables).
pub fn request_elevator(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::KeyL) {
        next_state.set(GameState::ElevatorTransition);
    }
}

/// Menu / ending / elevator-skip keyboard flow.
pub fn flow_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut floor: ResMut<CurrentFloor>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    match state.get() {
        GameState::MainMenu => {
            if keys.just_pressed(KeyCode::Enter) {
                next_state.set(GameState::InGame);
            }
            if keys.just_pressed(KeyCode::Escape) {
                exit.write(AppExit::Success);
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
        GameState::InGame => {}
    }
}
