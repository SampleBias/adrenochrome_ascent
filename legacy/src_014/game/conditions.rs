use bevy::prelude::*;

/// Returns true if the game is in any playable level (1..=7).
/// Used as a run condition to avoid verbose `.or()` chains.
pub fn in_any_level(state: Res<State<crate::game::states::GameState>>) -> bool {
    state.get().level_number().is_some()
}

/// Returns true if the game is in a menu state (main menu, paused, game over, victory).
pub fn in_menu(state: Res<State<crate::game::states::GameState>>) -> bool {
    use crate::game::states::GameState;
    matches!(
        state.get(),
        GameState::MainMenu | GameState::Paused | GameState::GameOver | GameState::Victory
    )
}
