//! Adrenochrome Ascent — Gameplay crate.
//!
//! ECS gameplay systems: player controller, game state machine, enemies,
//! puzzles, and combat. Operates on the engine raycaster map grid.
//!
//! TODO-004: Doom-style first-person controller.
//! TODO-005: GameState flow (MainMenu → InGame → ElevatorTransition → Ending).

use bevy::prelude::*;

use adrenochrome_engine::RaycasterSystems;

pub mod game;
pub mod player;
pub mod ui;

pub use game::{CurrentFloor, EndingKind, GameState};
pub use player::{Player, PlayerMotor, PlayerSet};

/// Gameplay plugin: state machine + player controller.
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<CurrentFloor>()
            .init_resource::<EndingKind>()
            .init_resource::<game::ElevatorTimer>()
            .configure_sets(
                Update,
                (
                    PlayerSet::Input,
                    PlayerSet::Move,
                    PlayerSet::Present,
                )
                    .chain()
                    .before(RaycasterSystems::Render)
                    .run_if(in_state(GameState::InGame)),
            )
            // --- MainMenu ---
            .add_systems(OnEnter(GameState::MainMenu), (
                game::enter_main_menu_reset,
                game::release_mouse,
                ui::spawn_main_menu,
            ))
            // --- InGame ---
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    game::apply_floor_palette,
                    game::enter_in_game_spawn_player,
                    player::capture_mouse,
                    ui::spawn_ingame_hud,
                )
                    .chain(),
            )
            .add_systems(OnExit(GameState::InGame), game::release_mouse)
            // --- Elevator ---
            .add_systems(
                OnEnter(GameState::ElevatorTransition),
                (
                    game::enter_elevator,
                    game::release_mouse,
                    ui::spawn_elevator_overlay,
                ),
            )
            .add_systems(
                Update,
                game::tick_elevator.run_if(in_state(GameState::ElevatorTransition)),
            )
            // --- Ending ---
            .add_systems(
                OnEnter(GameState::Ending),
                (game::release_mouse, ui::spawn_ending),
            )
            // --- Always-on flow input ---
            .add_systems(Update, game::flow_input)
            // --- In-game only ---
            .add_systems(
                Update,
                (
                    player::toggle_cursor_grab.in_set(PlayerSet::Input),
                    player::player_look.in_set(PlayerSet::Input),
                    player::player_move.in_set(PlayerSet::Move),
                    player::apply_hand_pitch.in_set(PlayerSet::Present),
                    game::request_elevator,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
