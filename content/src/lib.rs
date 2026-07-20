//! Adrenochrome Ascent — Content crate.
//!
//! Floor RON schemas and loaders (TODO-006). Gameplay's floor loader consumes
//! these definitions to populate the raycaster map and spawn entities.

use bevy::prelude::*;

pub mod floor_def;

pub use floor_def::{
    load_floor_file, load_floor_number, EnemyArchetypeId, EntityDef, EntityKind, FactionId,
    FloorCluster, FloorDef, InteractAction, PaletteId, PuzzleEffectId,
};

/// Content plugin: currently data-only; floor loading lives in gameplay.
pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, _app: &mut App) {}
}
