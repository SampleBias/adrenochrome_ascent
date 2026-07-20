//! CRT upscale material: samples the 320×200 render target and draws it
//! fullscreen with nearest-neighbor filtering, palette tint, and lo-fi
//! horror post (scanlines, vignette, dither, grain, pain/serum).
//!
//! Pipeline:
//!
//! ```text
//!   game scene ──> Camera2d (order=-1, RenderTarget::Image) ──> 320×200 Image
//!                                                                    │
//!                                                                    ▼
//!   window     <── Camera2d (order=0, fullscreen quad + CrtMaterial) ─┘
//! ```
//!
//! Style target: `assets/images/style_reference/`.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d},
};

use crate::palette::ActivePalette;

/// Embedded-asset path for the CRT upscale WGSL shader.
const CRT_SHADER_PATH: &str = "embedded://adrenochrome_engine/shaders/crt_upscale.wgsl";

/// Default CRT look tuned to the style-reference screenshots.
pub const DEFAULT_SCANLINE: f32 = 0.42;
pub const DEFAULT_VIGNETTE: f32 = 0.85;
pub const DEFAULT_DITHER: f32 = 0.75;
pub const DEFAULT_PHOSPHOR: f32 = 0.55;

/// Fullscreen material that upscales the low-res render target.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CrtMaterial {
    /// The low-res render target sampled with nearest-neighbor filtering.
    #[texture(0)]
    #[sampler(1)]
    pub source_texture: Handle<Image>,
    /// Linear RGBA tint (palette swap). Bound as a vec4 uniform at set 0, binding 2.
    #[uniform(2)]
    pub palette_tint: [f32; 4],
    /// CRT post params: `[scanline, vignette, dither, time_secs]`.
    #[uniform(3)]
    pub crt_params: [f32; 4],
    /// Extra post: `[pain, serum, phosphor, unused]`.
    #[uniform(4)]
    pub post_fx: [f32; 4],
}

impl CrtMaterial {
    pub fn new(source_texture: Handle<Image>, palette_tint: [f32; 4]) -> Self {
        Self {
            source_texture,
            palette_tint,
            crt_params: [DEFAULT_SCANLINE, DEFAULT_VIGNETTE, DEFAULT_DITHER, 0.0],
            post_fx: [0.0, 0.0, DEFAULT_PHOSPHOR, 0.0],
        }
    }
}

impl Material2d for CrtMaterial {
    fn fragment_shader() -> ShaderRef {
        CRT_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

/// Marker on the main (window) camera that renders the fullscreen upscale quad.
#[derive(Component)]
pub struct UpscaleCamera;

/// Marker on the fullscreen CRT quad so resize logic does not touch scene meshes.
#[derive(Component)]
pub struct CrtFullscreenQuad;

/// Each frame, push the active palette tint into every `CrtMaterial` so a
/// palette swap (e.g. on elevator transition) takes effect immediately.
pub fn update_crt_palette(
    active: Res<ActivePalette>,
    mut materials: ResMut<Assets<CrtMaterial>>,
) {
    if !active.is_changed() {
        return;
    }
    let tint = active.palette.tint();
    for (_, mat) in materials.iter_mut() {
        mat.palette_tint = tint;
    }
}

/// Animate CRT grain / subtle temporal noise via `crt_params.w`.
pub fn update_crt_time(time: Res<Time>, mut materials: ResMut<Assets<CrtMaterial>>) {
    let t = time.elapsed_secs();
    for (_, mat) in materials.iter_mut() {
        mat.crt_params[3] = t;
    }
}

/// Drive pain / serum uniforms from gameplay (TODO-034).
pub fn set_crt_post_fx(materials: &mut Assets<CrtMaterial>, pain: f32, serum: f32) {
    for (_, mat) in materials.iter_mut() {
        mat.post_fx[0] = pain.clamp(0.0, 1.0);
        mat.post_fx[1] = serum.clamp(0.0, 1.0);
        mat.post_fx[2] = DEFAULT_PHOSPHOR;
    }
}
