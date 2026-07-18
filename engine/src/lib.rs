//! Adrenochrome Ascent — Engine crate.
//!
//! Contains the raycaster rendering pipeline, 320×200 render target,
//! CRT shader, and billboard sprite system. Visual style targets the
//! lo-fi horror references in `assets/images/style_reference/`.

use bevy::{asset::embedded_asset, prelude::*};

pub mod crt_material;
pub mod demo;
pub mod palette;
pub mod render_target;

pub use crt_material::{
    CrtFullscreenQuad, CrtMaterial, LowResSceneCamera, UpscaleCamera, update_crt_palette,
    update_crt_time,
};
pub use palette::{ActivePalette, Palette, RENDER_HEIGHT, RENDER_WIDTH};
pub use render_target::{LowResTarget, fit_fullscreen_quad, setup_render_target};

/// Engine plugin: sets up the low-res CRT rendering pipeline.
pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shaders/crt_upscale.wgsl");

        app.add_plugins(bevy::sprite_render::Material2dPlugin::<CrtMaterial>::default())
            .add_systems(Startup, setup_render_target)
            .add_systems(
                Update,
                (
                    fit_fullscreen_quad,
                    update_crt_palette,
                    update_crt_time,
                    demo::demo_cycle_palette,
                ),
            )
            // Horror hallway mockup until the raycaster (TODO-003) lands.
            .add_systems(Startup, demo::setup_demo_content);
    }
}
