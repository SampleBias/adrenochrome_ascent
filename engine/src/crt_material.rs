//! CRT upscale material: samples the 320×200 render target and draws it
//! fullscreen with nearest-neighbor filtering + a palette tint.
//!
//! This is the second half of TODO-002. The pipeline is:
//!
//! ```text
//!   game scene ──> Camera2d (order=-1, RenderTarget::Image) ──> 320×200 Image
//!                                                                    │
//!                                                                    ▼
//!   window     <── Camera2d (order=0, fullscreen quad + CrtMaterial) ─┘
//! ```
//!
//! No CRT curvature/scanlines yet — those land in TODO-034. This material
//! only does the nearest-neighbor upscale and the palette-swap tint so that
//! floors can shift color grade (red → green → teal → black, TODO-006).

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d},
};

use crate::palette::ActivePalette;

/// Embedded-asset path for the CRT upscale WGSL shader.
///
/// The shader is registered via `embedded_asset!(app, "shaders/crt_upscale.wgsl")`
/// in [`crate::EnginePlugin::build`]. The `embedded://adrenochrome_engine/...`
/// prefix is the standard Bevy embedded-asset URL: `<crate_name>` is derived
/// from `module_path!()` at the `embedded_asset!` call site, and the rest is
/// the path relative to `engine/src/`.
const CRT_SHADER_PATH: &str = "embedded://adrenochrome_engine/shaders/crt_upscale.wgsl";

/// Fullscreen material that upscales the low-res render target.
///
/// `source_texture` is the 320×200 `Image` the game-scene camera renders into.
/// `palette_tint` is the active floor-cluster tint, refreshed each frame from
/// the [`ActivePalette`] resource by [`update_crt_palette`].
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CrtMaterial {
    /// The low-res render target sampled with nearest-neighbor filtering.
    #[texture(0)]
    #[sampler(1)]
    pub source_texture: Handle<Image>,
    /// Linear RGBA tint (palette swap). Bound as a vec4 uniform at set 0, binding 2.
    #[uniform(2)]
    pub palette_tint: [f32; 4],
}

impl Material2d for CrtMaterial {
    fn fragment_shader() -> ShaderRef {
        // `ShaderRef::Path` is resolved by the asset server during pipeline
        // specialization (see `Material2dPlugin`'s `get_shader`). The
        // `embedded://` scheme points at the asset registered by
        // `embedded_asset!` in `EnginePlugin::build`.
        CRT_SHADER_PATH.into()
    }

    // The fullscreen quad is fully opaque; no blending needed.
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}

/// Marker on the main (window) camera that renders the fullscreen upscale quad.
#[derive(Component)]
pub struct UpscaleCamera;

/// Marker on the low-res camera that renders the game scene into the 320×200
/// render target. The raycaster (TODO-003) will write into the image this
/// camera targets.
#[derive(Component)]
pub struct LowResSceneCamera;

/// Each frame, push the active palette tint into every `CrtMaterial` so a
/// palette swap (e.g. on elevator transition) takes effect immediately.
pub fn update_crt_palette(
    active: Res<ActivePalette>,
    mut materials: ResMut<Assets<CrtMaterial>>,
) {
    // Only react when the palette actually changed.
    if !active.is_changed() {
        return;
    }
    let tint = active.palette.tint();
    for (_, mat) in materials.iter_mut() {
        mat.palette_tint = tint;
    }
}
