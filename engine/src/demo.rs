//! First-person horror hallway mockup for the CRT pipeline.
//!
//! Placeholder until the raycaster (TODO-003) writes real world geometry.
//! Composition mirrors `assets/images/style_reference/`: long corridor,
//! darkness-forward lighting, distant red exit, silhouette figure, and a
//! neon magenta player hand in the foreground.
//!
//! Space cycles floor palettes (Red → Green → Teal → Black).

use bevy::prelude::*;

use crate::palette::{ActivePalette, RENDER_HEIGHT, RENDER_WIDTH};

/// Marker for demo entities so they can be cleaned up later.
#[derive(Component)]
pub struct DemoContent;

fn rect(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    w: f32,
    h: f32,
    color: Color,
    x: f32,
    y: f32,
    z: f32,
) {
    commands.spawn((
        DemoContent,
        Mesh2d(meshes.add(Rectangle::new(w, h))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
        Transform::from_xyz(x, y, z),
    ));
}

/// Spawn a first-person corridor scene into the 320×200 view.
pub fn setup_demo_content(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let w = RENDER_WIDTH as f32;
    let h = RENDER_HEIGHT as f32;
    let half_w = w * 0.5;
    let half_h = h * 0.5;

    // --- Ceiling (sickly institutional dark) ---
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        w,
        half_h * 0.95,
        Color::srgb(0.06, 0.09, 0.07),
        0.0,
        half_h * 0.45,
        -10.0,
    );

    // --- Floor base (murky green / blood hall) ---
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        w,
        half_h,
        Color::srgb(0.08, 0.10, 0.07),
        0.0,
        -half_h * 0.5,
        -10.0,
    );

    // Checkered floor tiles converging toward vanishing point (ref 04).
    let tile = 18.0;
    let mut row = 0;
    let mut y = -half_h + tile * 0.5;
    while y < -8.0 {
        let depth = ((y + half_h) / half_h).clamp(0.0, 1.0);
        let row_w = w * (0.35 + depth * 0.65);
        let cols = ((row_w / tile) as i32).max(2);
        let start_x = -row_w * 0.5 + tile * 0.5;
        for col in 0..cols {
            let light = (row + col) % 2 == 0;
            let shade = if light {
                Color::srgb(0.22, 0.20, 0.24)
            } else {
                Color::srgb(0.05, 0.05, 0.07)
            };
            // Magenta carpet wash on nearer tiles (ref 01).
            let color = if y > -half_h * 0.55 && light {
                Color::srgb(0.42, 0.12, 0.32)
            } else {
                shade
            };
            let scale = 0.55 + depth * 0.55;
            rect(
                &mut commands,
                &mut meshes,
                &mut materials,
                tile * scale,
                tile * 0.55 * scale,
                color,
                start_x + col as f32 * tile * (row_w / (cols as f32 * tile)),
                y,
                -9.0,
            );
        }
        y += tile * (0.45 + depth * 0.35);
        row += 1;
    }

    // Blood / rust stains on the floor (refs 02, 03, 05).
    for (x, y, s) in [
        (-40.0, -55.0, 28.0),
        (10.0, -70.0, 36.0),
        (55.0, -48.0, 22.0),
        (-15.0, -30.0, 18.0),
    ] {
        rect(
            &mut commands,
            &mut meshes,
            &mut materials,
            s,
            s * 0.35,
            Color::srgb(0.35, 0.05, 0.06),
            x,
            y,
            -8.5,
        );
    }

    // Left wall mass (converging).
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        90.0,
        h,
        Color::srgb(0.12, 0.14, 0.11),
        -half_w + 35.0,
        0.0,
        -8.0,
    );
    // Right wall mass.
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        90.0,
        h,
        Color::srgb(0.11, 0.13, 0.12),
        half_w - 35.0,
        0.0,
        -8.0,
    );

    // Wall paneling strips.
    for i in 0..5 {
        let t = i as f32 / 4.0;
        let x_l = -half_w + 20.0 + t * 55.0;
        let x_r = half_w - 20.0 - t * 55.0;
        let wall_h = h * (0.95 - t * 0.35);
        rect(
            &mut commands,
            &mut meshes,
            &mut materials,
            3.0,
            wall_h,
            Color::srgb(0.07, 0.08, 0.07),
            x_l,
            0.0,
            -7.5,
        );
        rect(
            &mut commands,
            &mut meshes,
            &mut materials,
            3.0,
            wall_h,
            Color::srgb(0.07, 0.08, 0.07),
            x_r,
            0.0,
            -7.5,
        );
    }

    // Ceiling light fixtures (pale green pools — refs 01, 04, 05).
    for (x, y, glow) in [
        (0.0, 72.0, Color::srgb(0.55, 0.85, 0.45)),
        (-50.0, 55.0, Color::srgb(0.35, 0.55, 0.30)),
        (50.0, 55.0, Color::srgb(0.35, 0.55, 0.30)),
    ] {
        rect(
            &mut commands,
            &mut meshes,
            &mut materials,
            34.0,
            8.0,
            glow,
            x,
            y,
            -6.0,
        );
        // Soft light pool on floor.
        rect(
            &mut commands,
            &mut meshes,
            &mut materials,
            50.0,
            14.0,
            Color::srgb(0.18, 0.28, 0.16),
            x * 0.4,
            -20.0 + y * -0.15,
            -8.8,
        );
    }

    // Floor lamps along the right wall (ref 04).
    for i in 0..3 {
        let t = i as f32;
        let x = 70.0 - t * 18.0;
        let y = -35.0 + t * 12.0;
        rect(
            &mut commands,
            &mut meshes,
            &mut materials,
            4.0,
            18.0 - t * 3.0,
            Color::srgb(0.15, 0.14, 0.10),
            x,
            y,
            -6.5,
        );
        rect(
            &mut commands,
            &mut meshes,
            &mut materials,
            8.0,
            4.0,
            Color::srgb(0.85, 0.75, 0.35),
            x,
            y + 10.0 - t,
            -6.4,
        );
    }

    // Distant double doors + ominous red exit glow (ref 05).
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        48.0,
        56.0,
        Color::srgb(0.55, 0.08, 0.08),
        0.0,
        8.0,
        -5.0,
    );
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        40.0,
        48.0,
        Color::srgb(0.08, 0.04, 0.05),
        0.0,
        8.0,
        -4.9,
    );
    // Door split.
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        2.0,
        48.0,
        Color::srgb(0.25, 0.06, 0.06),
        0.0,
        8.0,
        -4.8,
    );
    // Glass panes glowing red.
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        12.0,
        10.0,
        Color::srgb(0.95, 0.20, 0.18),
        -10.0,
        18.0,
        -4.7,
    );
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        12.0,
        10.0,
        Color::srgb(0.95, 0.20, 0.18),
        10.0,
        18.0,
        -4.7,
    );

    // Silhouette walking away (refs 03, 05).
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        10.0,
        28.0,
        Color::srgb(0.02, 0.02, 0.03),
        -6.0,
        -2.0,
        -4.0,
    );
    // Head.
    commands.spawn((
        DemoContent,
        Mesh2d(meshes.add(bevy::math::primitives::Circle::new(5.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(
            0.02, 0.02, 0.03,
        )))),
        Transform::from_xyz(-6.0, 16.0, -3.9),
    ));
    // Bright red shirt accent (ref 03) — reads through palette tint.
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        9.0,
        12.0,
        Color::srgb(0.85, 0.12, 0.10),
        -6.0,
        2.0,
        -3.8,
    );

    // Far figure behind glass (ref 05).
    rect(
        &mut commands,
        &mut meshes,
        &mut materials,
        6.0,
        14.0,
        Color::srgb(0.15, 0.02, 0.02),
        4.0,
        12.0,
        -4.6,
    );

    // Spectral cyan wisp / flame (ref 02).
    commands.spawn((
        DemoContent,
        Mesh2d(meshes.add(bevy::math::primitives::Circle::new(14.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(
            0.25, 0.85, 0.95,
        )))),
        Transform::from_xyz(55.0, -25.0, -3.0),
    ));
    commands.spawn((
        DemoContent,
        Mesh2d(meshes.add(bevy::math::primitives::Circle::new(8.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(
            0.55, 0.95, 1.0,
        )))),
        Transform::from_xyz(58.0, -18.0, -2.9),
    ));

    // Glitch static streaks near hand (ref 05).
    for (x, y, w, h) in [
        (35.0, -72.0, 40.0, 3.0),
        (50.0, -78.0, 22.0, 2.0),
        (20.0, -68.0, 16.0, 2.0),
    ] {
        rect(
            &mut commands,
            &mut meshes,
            &mut materials,
            w,
            h,
            Color::srgb(0.95, 0.35, 0.85),
            x,
            y,
            -1.5,
        );
    }

    // --- Player hand (neon magenta / pink outline — refs 01, 04, 05) ---
    spawn_player_hand(&mut commands, &mut meshes, &mut materials);
}

fn spawn_player_hand(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) {
    // Palm block (bottom-right, reaching in).
    let outline = Color::srgb(0.95, 0.25, 0.75);
    let fill = Color::srgb(0.04, 0.02, 0.05);

    // Outline (slightly larger).
    rect(
        commands,
        meshes,
        materials,
        58.0,
        42.0,
        outline,
        95.0,
        -78.0,
        5.0,
    );
    // Fill.
    rect(
        commands,
        meshes,
        materials,
        50.0,
        34.0,
        fill,
        95.0,
        -78.0,
        5.1,
    );

    // Fingers.
    for (dx, dy, fw, fh) in [
        (-18.0, 22.0, 12.0, 28.0),
        (-4.0, 26.0, 11.0, 32.0),
        (10.0, 24.0, 11.0, 30.0),
        (22.0, 18.0, 10.0, 22.0),
    ] {
        rect(
            commands,
            meshes,
            materials,
            fw + 4.0,
            fh + 4.0,
            outline,
            95.0 + dx,
            -78.0 + dy,
            5.2,
        );
        rect(
            commands,
            meshes,
            materials,
            fw,
            fh,
            fill,
            95.0 + dx,
            -78.0 + dy,
            5.3,
        );
    }

    // Inner glow veins / glitch (ref 04 pink hand).
    rect(
        commands,
        meshes,
        materials,
        20.0,
        8.0,
        Color::srgb(0.75, 0.15, 0.55),
        100.0,
        -75.0,
        5.4,
    );
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
