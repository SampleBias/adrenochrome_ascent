//! 320×200 CPU framebuffer + CRT upscale camera setup.
//!
//! The software raycaster writes BGRA pixels into [`LowResTarget`] each frame.
//! An `UpscaleCamera` draws a fullscreen [`CrtMaterial`] quad that samples the
//! buffer with nearest-neighbor filtering (scanlines / vignette / palette tint).

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use crate::{
    crt_material::{CrtFullscreenQuad, CrtMaterial, CrtMaterialHandle, UpscaleCamera},
    palette::{ActivePalette, Palette, RENDER_HEIGHT, RENDER_WIDTH},
};

/// Resource handle to the 320×200 CPU/GPU framebuffer image.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct LowResTarget(pub Handle<Image>);

/// Spawns the framebuffer image, upscale camera, and CRT fullscreen quad.
pub fn setup_render_target(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CrtMaterial>>,
) {
    // CPU-writable framebuffer. The raycaster mutates `Image::data` each frame;
    // Bevy re-uploads when the asset is marked changed via `Assets::get_mut`.
    let mut target = Image::new_fill(
        Extent3d {
            width: RENDER_WIDTH,
            height: RENDER_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255], // BGRA black
        TextureFormat::Bgra8UnormSrgb,
        bevy::asset::RenderAssetUsages::default(),
    );
    target.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
    target.sampler = bevy::image::ImageSampler::nearest();

    let target_handle = images.add(target);
    commands.insert_resource(LowResTarget(target_handle.clone()));

    let tint = Palette::Red.tint();
    let crt_material = materials.add(CrtMaterial::new(target_handle, tint));
    commands.insert_resource(CrtMaterialHandle(crt_material.clone()));
    let quad = meshes.add(Rectangle::default());

    commands.spawn((
        Name::new("UpscaleCamera"),
        Camera2d,
        Camera {
            order: 0,
            clear_color: Color::srgb(0.0, 0.0, 0.0).into(),
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
