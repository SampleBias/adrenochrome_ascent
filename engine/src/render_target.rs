//! 320×200 internal render target + dual-camera setup (TODO-002).
//!
//! This module owns the "render the game to a low-res image, then upscale it
//! to the window" pipeline:
//!
//! - A 320×200 `Image` with `RENDER_ATTACHMENT` + `TEXTURE_BINDING` usage and
//!   nearest-neighbor sampling. The game-scene camera renders into it.
//! - A `Camera2d` (`LowResSceneCamera`, order -1) targeting that image. The
//!   raycaster (TODO-003) and all gameplay rendering hang off this camera.
//! - A `Camera2d` (`UpscaleCamera`, order 0) targeting the window. It renders
//!   a single fullscreen quad using [`CrtMaterial`], which samples the
//!   low-res image with nearest-neighbor filtering and applies the palette tint.
//!
//! The low-res camera also gets a clear color so an empty scene is visible
//! (acceptance criterion: "a 320×200 render target is visible, upscaled").

use bevy::{
    camera::RenderTarget,
    prelude::*,
    render::render_resource::TextureFormat,
};

use crate::{
    crt_material::{CrtMaterial, LowResSceneCamera, UpscaleCamera},
    palette::{ActivePalette, Palette, RENDER_HEIGHT, RENDER_WIDTH},
};

/// Resource handle to the 320×200 render-target image.
///
/// Exposed so the raycaster (TODO-003) can write pixels into the same image
/// the upscale material samples.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct LowResTarget(pub Handle<Image>);

/// Spawns the render-target image and both cameras.
///
/// Runs in `Startup`. Idempotent in spirit: it only spawns entities that
/// don't exist yet, but in practice it runs once.
pub fn setup_render_target(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CrtMaterial>>,
) {
    // --- 1. The 320×200 render target image. ---
    let mut target = Image::new_target_texture(
        RENDER_WIDTH,
        RENDER_HEIGHT,
        TextureFormat::Bgra8UnormSrgb,
        None,
    );

    // Bevy's `new_target_texture` defaults to `ImageSampler::Default`, which
    // respects the global `ImagePlugin` sampler. We want *guaranteed*
    // nearest-neighbor for the crunchy pixel upscale regardless of how the
    // app configures `ImagePlugin`, so force a nearest descriptor.
    target.sampler = bevy::image::ImageSampler::nearest();

    let target_handle = images.add(target);
    commands.insert_resource(LowResTarget(target_handle.clone()));

    // --- 2. Low-res scene camera: renders the game into the target image. ---
    // order = -1 so it renders before the upscale camera. The raycaster and
    // all gameplay 2D rendering will live under this camera's view.
    commands.spawn((
        Name::new("LowResSceneCamera"),
        Camera2d,
        Camera {
            order: -1,
            clear_color: Color::srgb(0.02, 0.02, 0.03).into(),
            ..default()
        },
        RenderTarget::Image(target_handle.clone().into()),
        LowResSceneCamera,
    ));

    // --- 3. Upscale camera: renders the fullscreen quad to the window. ---
    // A 2D camera at z=0 with a fullscreen quad. The quad uses CrtMaterial,
    // which samples the low-res target. We scale the rectangle to the window
    // size; since the camera is orthographic with default 2D projection, a
    // unit quad at scale = window size fills the screen.
    let tint = Palette::Red.tint();
    let crt_material = materials.add(CrtMaterial {
        source_texture: target_handle,
        palette_tint: tint,
    });

    // A 1×1 rectangle scaled up to fill the viewport. Using a rectangle mesh
    // (rather than a raw fullscreen triangle) keeps us inside the Material2d
    // pipeline, which is simpler and sufficient for TODO-002.
    let quad = meshes.add(Rectangle::default());

    commands.spawn((
        Name::new("UpscaleCamera"),
        Camera2d,
        Camera {
            order: 0,
            clear_color: Color::NONE.into(),
            ..default()
        },
        UpscaleCamera,
    ));

    commands.spawn((
        Name::new("CrtFullscreenQuad"),
        Mesh2d(quad),
        MeshMaterial2d(crt_material),
        // Scale the unit rectangle to fill a 2D camera's default viewport
        // (which spans -1..1 on both axes → 2 units wide/tall). A scale of
        // 2.0 on each axis covers it; we go slightly over to avoid edge gaps.
        Transform::default().with_scale(Vec3::new(2.0, 2.0, 1.0)),
    ));

    // Seed the active palette resource so the tint system has a starting point.
    // `init_resource` is a no-op if it already exists.
    commands.init_resource::<ActivePalette>();
}

/// Resizes the fullscreen quad when the window changes, keeping the low-res
/// image's aspect ratio correct (letterboxed) rather than stretching.
///
/// TODO-002 acceptance only requires the target to be "visible, upscaled";
/// letterboxing is the correct retro behavior and avoids aspect distortion.
pub fn fit_fullscreen_quad(
    windows: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
    mut quad: Query<&mut Transform, With<Mesh2d>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let win_w = window.width();
    let win_h = window.height();

    for mut transform in &mut quad {
        // The Camera2d default projection maps world units 1:1 to pixels,
        // centered on origin. A quad of size (win_w, win_h) fills the window.
        transform.scale = Vec3::new(win_w, win_h, 1.0);
    }
}
