use bevy::prelude::*;

use crate::game::conditions::in_any_level;
use crate::game::states::GameState;
use crate::ui::hud::{despawn_hud, spawn_hud, update_hud};
use crate::ui::menus::{
    despawn_menus, menu_input, spawn_game_over, spawn_main_menu, spawn_pause_menu, spawn_victory,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app // Main menu
            .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_menus)
            // Pause
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), despawn_menus)
            // Game over
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over)
            .add_systems(OnExit(GameState::GameOver), despawn_menus)
            // Victory
            .add_systems(OnEnter(GameState::Victory), spawn_victory)
            .add_systems(OnExit(GameState::Victory), despawn_menus)
            // HUD: spawn on each level enter, despawn on exit.
            .add_systems(OnEnter(GameState::Level1), spawn_hud)
            .add_systems(OnEnter(GameState::Level2), spawn_hud)
            .add_systems(OnEnter(GameState::Level3), spawn_hud)
            .add_systems(OnEnter(GameState::Level4), spawn_hud)
            .add_systems(OnEnter(GameState::Level5), spawn_hud)
            .add_systems(OnEnter(GameState::Level6), spawn_hud)
            .add_systems(OnEnter(GameState::Level7), spawn_hud)
            .add_systems(OnExit(GameState::Level1), despawn_hud)
            .add_systems(OnExit(GameState::Level2), despawn_hud)
            .add_systems(OnExit(GameState::Level3), despawn_hud)
            .add_systems(OnExit(GameState::Level4), despawn_hud)
            .add_systems(OnExit(GameState::Level5), despawn_hud)
            .add_systems(OnExit(GameState::Level6), despawn_hud)
            .add_systems(OnExit(GameState::Level7), despawn_hud)
            // Per-frame
            .add_systems(Update, update_hud.run_if(in_any_level))
            .add_systems(Update, menu_input);
    }
}
