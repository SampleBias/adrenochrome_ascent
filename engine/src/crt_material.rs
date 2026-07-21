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

/// Default CRT look — readable playfield, dirt/atmosphere on the rim.
pub const DEFAULT_SCANLINE: f32 = 0.32;
pub const DEFAULT_VIGNETTE: f32 = 0.78;
pub const DEFAULT_DITHER: f32 = 0.55;
pub const DEFAULT_PHOSPHOR: f32 = 0.55;

/// Fullscreen material that upscales the low-res render target.
///
/// Uniforms use [`Vec4`] (WGSL `vec4<f32>`), not `[f32; 4]` — encase treats
/// Rust arrays as WGSL arrays whose stride must be a multiple of 16.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CrtMaterial {
    /// The low-res render target sampled with nearest-neighbor filtering.
    #[texture(0)]
    #[sampler(1)]
    pub source_texture: Handle<Image>,
    /// Linear RGBA tint (palette swap). Bound as a vec4 uniform at set 0, binding 2.
    #[uniform(2)]
    pub palette_tint: Vec4,
    /// CRT post params: `x=scanline, y=vignette, z=dither, w=time_secs`.
    #[uniform(3)]
    pub crt_params: Vec4,
    /// Extra post: `x=pain, y=serum, z=phosphor, w=unused`.
    #[uniform(4)]
    pub post_fx: Vec4,
}

impl CrtMaterial {
    pub fn new(source_texture: Handle<Image>, palette_tint: [f32; 4]) -> Self {
        Self {
            source_texture,
            palette_tint: Vec4::from_array(palette_tint),
            crt_params: Vec4::new(DEFAULT_SCANLINE, DEFAULT_VIGNETTE, DEFAULT_DITHER, 0.0),
            post_fx: Vec4::new(0.0, 0.0, DEFAULT_PHOSPHOR, 0.0),
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

/// Handle to the live CRT upscale material (avoids `Assets::iter_mut` every frame).
#[derive(Resource, Clone, Debug)]
pub struct CrtMaterialHandle(pub Handle<CrtMaterial>);

/// Each frame, push the active palette tint into every `CrtMaterial` so a
/// palette swap (e.g. on elevator transition) takes effect immediately.
pub fn update_crt_palette(
    active: Res<ActivePalette>,
    handle: Option<Res<CrtMaterialHandle>>,
    mut materials: ResMut<Assets<CrtMaterial>>,
) {
    if !active.is_changed() {
        return;
    }
    let Some(handle) = handle else {
        return;
    };
    let tint = Vec4::from_array(active.palette.tint());
    if let Some(mut mat) = materials.get_mut(&handle.0) {
        mat.palette_tint = tint;
    }
}

/// Animate CRT grain / subtle temporal noise via `crt_params.w`.
///
/// Throttled to ~20 Hz so we do not rebuild material bind groups every frame
/// (a common cause of swapchain acquire timeouts under load).
pub fn update_crt_time(
    time: Res<Time>,
    handle: Option<Res<CrtMaterialHandle>>,
    mut materials: ResMut<Assets<CrtMaterial>>,
    mut last: Local<f32>,
) {
    let t = time.elapsed_secs();
    if t - *last < 0.05 {
        return;
    }
    *last = t;
    let Some(handle) = handle else {
        return;
    };
    if let Some(mut mat) = materials.get_mut(&handle.0) {
        mat.crt_params.w = t;
    }
}

/// Drive pain / serum uniforms from gameplay (TODO-034).
pub fn set_crt_post_fx(materials: &mut Assets<CrtMaterial>, pain: f32, serum: f32) {
    for (_, mat) in materials.iter_mut() {
        mat.post_fx.x = pain.clamp(0.0, 1.0);
        mat.post_fx.y = serum.clamp(0.0, 1.0);
        mat.post_fx.z = DEFAULT_PHOSPHOR;
    }
}
