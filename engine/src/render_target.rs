//! 320×200 internal render target + dual-camera setup.
//!
//! This module owns the "render the game to a low-res image, then upscale it
//! to the window" pipeline:
//!
//! - A 320×200 `Image` with nearest-neighbor sampling.
//! - A `Camera2d` (`LowResSceneCamera`, order -1) targeting that image.
//! - A `Camera2d` (`UpscaleCamera`, order 0) targeting the window with a
//!   fullscreen [`CrtMaterial`] quad (scanlines / vignette / dither / tint).

use bevy::{
    camera::RenderTarget,
    prelude::*,
    render::render_resource::TextureFormat,
};

use crate::{
    crt_material::{CrtFullscreenQuad, CrtMaterial, LowResSceneCamera, UpscaleCamera},
    palette::{ActivePalette, Palette, RENDER_HEIGHT, RENDER_WIDTH},
};

/// Resource handle to the 320×200 render-target image.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct LowResTarget(pub Handle<Image>);

/// Spawns the render-target image and both cameras.
pub fn setup_render_target(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CrtMaterial>>,
) {
    let mut target = Image::new_target_texture(
        RENDER_WIDTH,
        RENDER_HEIGHT,
        TextureFormat::Bgra8UnormSrgb,
        None,
    );
    target.sampler = bevy::image::ImageSampler::nearest();

    let target_handle = images.add(target);
    commands.insert_resource(LowResTarget(target_handle.clone()));

    // Near-black clear — darkness-forward horror (style refs).
    commands.spawn((
        Name::new("LowResSceneCamera"),
        Camera2d,
        Camera {
            order: -1,
            clear_color: Color::srgb(0.01, 0.008, 0.012).into(),
            ..default()
        },
        RenderTarget::Image(target_handle.clone().into()),
        LowResSceneCamera,
    ));

    let tint = Palette::Red.tint();
    let crt_material = materials.add(CrtMaterial::new(target_handle, tint));
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
        CrtFullscreenQuad,
        Mesh2d(quad),
        MeshMaterial2d(crt_material),
        Transform::default().with_scale(Vec3::new(2.0, 2.0, 1.0)),
    ));

    commands.init_resource::<ActivePalette>();
}

/// Resizes the fullscreen CRT quad when the window changes.
pub fn fit_fullscreen_quad(
    windows: Query<&bevy::window::Window, With<bevy::window::PrimaryWindow>>,
    mut quad: Query<&mut Transform, With<CrtFullscreenQuad>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let win_w = window.width();
    let win_h = window.height();

    for mut transform in &mut quad {
        transform.scale = Vec3::new(win_w, win_h, 1.0);
    }
}
