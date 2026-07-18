//! Test map content for the raycaster (TODO-003).
//!
//! Spawns a horror hallway `MapGrid`, `RayCamera`, billboard props, and the
//! hand overlay. Player movement lives in the gameplay crate (TODO-004).
//! Space still cycles floor palettes for CRT tint checks.

use bevy::prelude::*;

use crate::{
    billboard::{Billboard, HandOverlay},
    map::MapGrid,
    palette::ActivePalette,
    ray_camera::RayCamera,
};

/// Horror test hallway — style refs: long corridor, door at end, side rooms.
pub fn setup_test_map(mut commands: Commands) {
    let map = MapGrid::from_rows(&[
        "####################",
        "#..................#",
        "#..22......55......#",
        "#..22......55......#",
        "#..................#",
        "#........4.........#",
        "#..................#",
        "#.####........####.#",
        "#.#..#........#..#.#",
        "#.#..#...DD...#..#.#",
        "#.####........####.#",
        "#..................#",
        "#..MM..........MM..#",
        "#..................#",
        "####################",
    ]);

    let start = Vec2::new(10.5, 12.5);
    // Face toward decreasing Y (toward the door around y=9).
    let camera = RayCamera::from_yaw(start, -std::f32::consts::FRAC_PI_2);

    commands.insert_resource(map);
    commands.insert_resource(camera);

    commands.spawn((
        Name::new("BillboardEnemy"),
        Billboard::new(Vec2::new(10.5, 6.5), 0, 0.95),
    ));
    commands.spawn((
        Name::new("BillboardWisp"),
        Billboard::new(Vec2::new(14.5, 8.5), 2, 0.7),
    ));
    commands.spawn((
        Name::new("BillboardKeycard"),
        Billboard::new(Vec2::new(3.5, 8.5), 3, 0.45),
    ));
    commands.spawn((
        Name::new("BillboardFarFigure"),
        Billboard::new(Vec2::new(10.5, 9.2), 0, 0.7),
    ));

    commands.spawn((Name::new("HandOverlay"), HandOverlay::default()));
}

/// Press Space to cycle the active palette (Red → Green → Teal → Black).
pub fn demo_cycle_palette(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active: ResMut<ActivePalette>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        active.palette = active.palette.next();
        info!("Palette swapped to: {:?}", active.palette);
    }
}
