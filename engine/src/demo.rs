//! Minimal demo content for TODO-002 acceptance.
//!
//! Spawns a few colored shapes into the low-res scene camera's view so the
//! 320×200 → window upscale pipeline is visibly working, and lets the user
//! cycle palettes with Space to verify the CRT tint shader.
//!
//! This is throwaway scaffolding — it will be removed once the raycaster
//! (TODO-003) writes real content into the render target.

use bevy::prelude::*;

use crate::palette::{ActivePalette, RENDER_HEIGHT, RENDER_WIDTH};

/// Marker for demo entities so they can be cleaned up later.
#[derive(Component)]
pub struct DemoContent;

/// Spawn a simple test pattern (a bright rectangle + circle) into the
/// low-res camera's view. The `Camera2d` default projection maps world
/// units 1:1 to the 320×200 render-target pixels, centered on origin.
pub fn setup_demo_content(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Background panel: a dark rectangle filling most of the target.
    commands.spawn((
        DemoContent,
        Mesh2d(meshes.add(Rectangle::new(
            RENDER_WIDTH as f32 * 0.8,
            RENDER_HEIGHT as f32 * 0.8,
        ))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(
            0.15, 0.12, 0.2,
        )))),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));

    // A bright accent rectangle so the palette tint is obvious.
    commands.spawn((
        DemoContent,
        Mesh2d(meshes.add(Rectangle::new(80.0, 40.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(
            0.9, 0.9, 0.9,
        )))),
        Transform::from_xyz(-60.0, 30.0, 0.0),
    ));

    // A circle to show curved edges (useful for spotting filtering issues).
    commands.spawn((
        DemoContent,
        Mesh2d(meshes.add(bevy::math::primitives::Circle::new(25.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(
            0.8, 0.7, 0.3,
        )))),
        Transform::from_xyz(60.0, -30.0, 0.0),
    ));
}

/// Press Space to cycle the active palette (Red → Green → Teal → Black).
///
/// Demonstrates the palette-swap tint described in TODO-006. The
/// `update_crt_palette` system picks up the change and pushes the new tint
/// into the `CrtMaterial`.
pub fn demo_cycle_palette(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active: ResMut<ActivePalette>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        active.palette = active.palette.next();
        info!("Palette swapped to: {:?}", active.palette);
    }
}
