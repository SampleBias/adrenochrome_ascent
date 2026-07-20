//! Adrenochrome Ascent — Engine crate.
//!
//! Software Doom-style raycaster → 320×200 CPU framebuffer → CRT upscale.
//! Visual style targets `assets/images/style_reference/`.

use bevy::{asset::embedded_asset, prelude::*};

pub mod attract;
pub mod billboard;
pub mod crt_material;
pub mod demo;
pub mod frame_source;
pub mod map;
pub mod palette;
pub mod pixel_hud;
pub mod ray_camera;
pub mod raycaster;
pub mod render_target;
pub mod textures;

pub use billboard::{Billboard, HandOverlay};
pub use crt_material::{
    set_crt_post_fx, update_crt_palette, update_crt_time, CrtFullscreenQuad, CrtMaterial,
    CrtMaterialHandle, UpscaleCamera, DEFAULT_DITHER, DEFAULT_SCANLINE, DEFAULT_VIGNETTE,
};
pub use frame_source::FrameSource;
pub use map::MapGrid;
pub use palette::{ActivePalette, Palette, RENDER_HEIGHT, RENDER_WIDTH};
pub use pixel_hud::PixelHud;
pub use ray_camera::RayCamera;
pub use raycaster::cast_ray;
pub use render_target::{fit_fullscreen_quad, setup_render_target, LowResTarget};
pub use textures::TextureSet;

/// Ordered raycaster stages so gameplay systems can run before the frame draw.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RaycasterSystems {
    Render,
}

/// Engine plugin: CRT pipeline + software raycaster.
pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "shaders/crt_upscale.wgsl");

        app.add_plugins(bevy::sprite_render::Material2dPlugin::<CrtMaterial>::default())
            .init_resource::<raycaster::DepthBuffer>()
            .init_resource::<PixelHud>()
            .init_resource::<FrameSource>()
            .insert_resource(TextureSet::procedural())
            // Defaults until gameplay floor loader replaces them.
            .insert_resource(MapGrid::from_rows(&["###", "#.#", "###"]))
            .insert_resource(RayCamera::default())
            .configure_sets(Update, RaycasterSystems::Render)
            .add_systems(Startup, setup_render_target)
            .add_systems(
                Update,
                (
                    demo::demo_cycle_palette,
                    raycaster::render_frame.in_set(RaycasterSystems::Render),
                    fit_fullscreen_quad,
                    update_crt_palette,
                    update_crt_time,
                ),
            );
    }
}
