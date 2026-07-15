//! Adrenochrome Ascent — Engine crate.
//!
//! Contains the raycaster rendering pipeline, 320×200 render target,
//! CRT shader, and billboard sprite system. This is the core rendering
//! engine that replaces the old Bevy 0.14 3D `PbrBundle`/`Camera3d` path.
//!
//! TODO-001: Workspace scaffold. Rendering systems are added in TODO-002
//! (render target + CRT shader) and TODO-003 (raycaster).

use bevy::prelude::*;

/// Engine plugin: sets up the raycaster rendering pipeline.
///
/// In TODO-001 this is a stub. TODO-002 adds the 320×200 render target
/// and upscale shader. TODO-003 adds the DDA raycaster.
pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, _app: &mut App) {
        // Rendering pipeline will be registered here in TODO-002/003.
    }
}
