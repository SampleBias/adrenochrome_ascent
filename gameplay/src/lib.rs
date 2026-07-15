//! Adrenochrome Ascent — Gameplay crate.
//!
//! Contains ECS gameplay systems: player controller, enemy AI, puzzle
//! interaction, game state machine, and combat. These systems operate
//! on the 2D map grid provided by the engine's raycaster.
//!
//! TODO-001: Workspace scaffold. Systems are added in TODO-004 (player
//! controller) and TODO-005 (state machine).

use bevy::prelude::*;

/// Gameplay plugin: registers all gameplay systems.
///
/// In TODO-001 this is a stub. TODO-004 adds the player controller,
/// TODO-005 adds the state machine, and subsequent sprints add
/// enemies, puzzles, and combat.
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, _app: &mut App) {
        // Gameplay systems will be registered here in TODO-004/005.
    }
}
