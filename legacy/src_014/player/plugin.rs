use bevy::prelude::*;

use crate::game::conditions::in_any_level;
use crate::game::states::GameState;
use crate::player::controller::{
    capture_mouse, despawn_player, player_movement, release_mouse, PlayerSet,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, PlayerSet)
            .add_systems(
                Update,
                player_movement.run_if(in_any_level).in_set(PlayerSet),
            )
            // Grab mouse on level enter, release on menus.
            .add_systems(OnEnter(GameState::Level1), capture_mouse)
            .add_systems(OnEnter(GameState::Level2), capture_mouse)
            .add_systems(OnEnter(GameState::Level3), capture_mouse)
            .add_systems(OnEnter(GameState::Level4), capture_mouse)
            .add_systems(OnEnter(GameState::Level5), capture_mouse)
            .add_systems(OnEnter(GameState::Level6), capture_mouse)
            .add_systems(OnEnter(GameState::Level7), capture_mouse)
            .add_systems(OnEnter(GameState::MainMenu), release_mouse)
            .add_systems(OnEnter(GameState::Paused), release_mouse)
            .add_systems(OnEnter(GameState::GameOver), release_mouse)
            .add_systems(OnEnter(GameState::Victory), release_mouse)
            // Despawn player when leaving levels.
            .add_systems(OnExit(GameState::Level1), despawn_player)
            .add_systems(OnExit(GameState::Level2), despawn_player)
            .add_systems(OnExit(GameState::Level3), despawn_player)
            .add_systems(OnExit(GameState::Level4), despawn_player)
            .add_systems(OnExit(GameState::Level5), despawn_player)
            .add_systems(OnExit(GameState::Level6), despawn_player)
            .add_systems(OnExit(GameState::Level7), despawn_player);
    }
}
