//! Adrenochrome Ascent — Engine crate.
//!
//! Contains the raycaster rendering pipeline, 320×200 render target,
//! CRT shader, and billboard sprite system. This is the core rendering
//! engine that replaces the old Bevy 0.14 3D `PbrBundle`/`Camera3d` path.
//!
//! TODO-001: Workspace scaffold (done).
//! TODO-002: 320×200 render target + nearest-neighbor upscale + palette
//!           swap shader (this commit).

use bevy::{asset::embedded_asset, prelude::*};

pub mod crt_material;
pub mod demo;
pub mod palette;
pub mod render_target;

pub use crt_material::{CrtMaterial, LowResSceneCamera, UpscaleCamera, update_crt_palette};
pub use palette::{ActivePalette, Palette, RENDER_HEIGHT, RENDER_WIDTH};
pub use render_target::{LowResTarget, fit_fullscreen_quad, setup_render_target};

/// Engine plugin: sets up the raycaster rendering pipeline.
///
/// TODO-002 registers the CRT upscale material, spawns the 320×200 render
/// target + dual cameras, and runs the palette-tint + fullscreen-fit systems.
/// TODO-003 will add the DDA raycaster that writes into the low-res target.
pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        // Embed the CRT upscale WGSL shader into the binary so the engine
        // crate is self-contained (no `assets/` dependency for a core
        // pipeline). The shader becomes loadable at
        // `embedded://adrenochrome_engine/shaders/crt_upscale.wgsl`, which
        // `CrtMaterial::fragment_shader` references.
        embedded_asset!(app, "shaders/crt_upscale.wgsl");

        // Register the CRT upscale material so `Assets<CrtMaterial>` and the
        // 2D render pipeline for it exist.
        app.add_plugins(bevy::sprite_render::Material2dPlugin::<CrtMaterial>::default())
            // Spawn the render target image + low-res camera + upscale camera
            // + fullscreen quad.
            .add_systems(Startup, setup_render_target)
            // Per-frame: keep the fullscreen quad sized to the window, and
            // push palette changes into the CRT material.
            .add_systems(
                Update,
                (
                    fit_fullscreen_quad,
                    update_crt_palette,
                    demo::demo_cycle_palette,
                ),
            )
            // Throwaway demo content so the 320×200 → window pipeline is
            // visibly working (TODO-002 acceptance). Removed once the
            // raycaster (TODO-003) writes real content.
            .add_systems(Startup, demo::setup_demo_content);
    }
}
