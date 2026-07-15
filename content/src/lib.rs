//! Adrenochrome Ascent — Content crate.
//!
//! Contains floor data structures, RON floor definitions, asset manifests,
//! and palette definitions. This crate is data-heavy and code-light —
//! it provides typed structures that the gameplay crate loads at runtime.
//!
//! TODO-001: Workspace scaffold. Floor RON files are added in TODO-006.

use bevy::prelude::*;

/// Content plugin: registers asset loaders and floor data resources.
///
/// In TODO-001 this is a stub. TODO-006 adds floor RON structures,
/// TODO-007 adds the floor loader system.
pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, _app: &mut App) {
        // Content/asset systems will be registered here in TODO-006/007.
    }
}
