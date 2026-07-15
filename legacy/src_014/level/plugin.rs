use bevy::prelude::*;

use crate::game::states::GameState;
use crate::level::loader::{
    enter_level1, enter_level2, enter_level3, enter_level4, enter_level5, enter_level6,
    enter_level7, unload_level,
};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app // Enter
            .add_systems(OnEnter(GameState::Level1), enter_level1)
            .add_systems(OnEnter(GameState::Level2), enter_level2)
            .add_systems(OnEnter(GameState::Level3), enter_level3)
            .add_systems(OnEnter(GameState::Level4), enter_level4)
            .add_systems(OnEnter(GameState::Level5), enter_level5)
            .add_systems(OnEnter(GameState::Level6), enter_level6)
            .add_systems(OnEnter(GameState::Level7), enter_level7)
            // Exit (unload geometry + puzzle entities; player despawn is in PlayerPlugin)
            .add_systems(OnExit(GameState::Level1), unload_level)
            .add_systems(OnExit(GameState::Level2), unload_level)
            .add_systems(OnExit(GameState::Level3), unload_level)
            .add_systems(OnExit(GameState::Level4), unload_level)
            .add_systems(OnExit(GameState::Level5), unload_level)
            .add_systems(OnExit(GameState::Level6), unload_level)
            .add_systems(OnExit(GameState::Level7), unload_level);
    }
}
