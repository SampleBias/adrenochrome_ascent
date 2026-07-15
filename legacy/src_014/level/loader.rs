use bevy::prelude::*;

use crate::game::constants::{FLOOR_COLOR, FLOOR_SIZE, WALL_COLOR, WALL_HEIGHT};
use crate::game::plugin::CurrentLevelInfo;
use crate::game::states::GameState;
use crate::level::definitions::LevelDefinition;
use crate::player::controller::spawn_player;
use crate::puzzle::components::PuzzleInteractable;

/// Marker for all entities spawned by the level loader, so we can despawn
/// them cleanly on exit.
#[derive(Component, Debug, Clone, Copy)]
pub struct LevelEntity;

/// Spawns the geometry, lights, player, and puzzle interactables for a level.
///
/// This is a generic loader driven by `LevelDefinition`. Per-level bespoke
/// content (enemy patrols, specific puzzle layouts) is added by dedicated
/// systems gated on the specific `GameState`.
pub fn load_level(state: GameState, world: &mut World) {
    let Some(def) = LevelDefinition::for_state(state) else {
        return;
    };

    // Store level info for the HUD.
    world.insert_resource(CurrentLevelInfo {
        number: def.number,
        name: def.name.to_string(),
        subtitle: def.subtitle.to_string(),
    });

    // Spawn player.
    spawn_player(&mut world.commands(), def.player_spawn);

    // --- Create shared assets up front ---
    let floor_mesh = world
        .resource_mut::<Assets<Mesh>>()
        .add(Plane3d::new(Vec3::Y, Vec2::splat(FLOOR_SIZE / 2.0)));
    let wall_mesh =
        world
            .resource_mut::<Assets<Mesh>>()
            .add(Cuboid::new(FLOOR_SIZE, WALL_HEIGHT, 0.2));
    let floor_mat = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::srgb(FLOOR_COLOR[0], FLOOR_COLOR[1], FLOOR_COLOR[2]),
            ..default()
        });
    let wall_mat = world
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            base_color: Color::srgb(WALL_COLOR[0], WALL_COLOR[1], WALL_COLOR[2]),
            ..default()
        });

    // Floor.
    world.spawn((
        Name::new("Floor"),
        LevelEntity,
        PbrBundle {
            mesh: floor_mesh.clone(),
            material: floor_mat.clone(),
            ..default()
        },
    ));

    // Ceiling.
    world.spawn((
        Name::new("Ceiling"),
        LevelEntity,
        PbrBundle {
            mesh: floor_mesh,
            material: wall_mat.clone(),
            transform: Transform::from_xyz(0.0, WALL_HEIGHT, 0.0)
                .with_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
            ..default()
        },
    ));

    // Four walls.
    let half = FLOOR_SIZE / 2.0;
    for (pos, rot) in [
        (Vec3::new(0.0, WALL_HEIGHT / 2.0, -half), Quat::IDENTITY),
        (
            Vec3::new(0.0, WALL_HEIGHT / 2.0, half),
            Quat::from_rotation_y(std::f32::consts::PI),
        ),
        (
            Vec3::new(-half, WALL_HEIGHT / 2.0, 0.0),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        ),
        (
            Vec3::new(half, WALL_HEIGHT / 2.0, 0.0),
            Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
        ),
    ] {
        world.spawn((
            Name::new("Wall"),
            LevelEntity,
            PbrBundle {
                mesh: wall_mesh.clone(),
                material: wall_mat.clone(),
                transform: Transform::from_translation(pos).with_rotation(rot),
                ..default()
            },
        ));
    }

    // Ambient light (a global resource in Bevy 0.14).
    world.insert_resource(AmbientLight {
        color: Color::srgb(
            def.ambient_light[0],
            def.ambient_light[1],
            def.ambient_light[2],
        ),
        brightness: def.ambient_brightness,
    });

    // A point light near the center for visibility.
    world.spawn((
        Name::new("CeilingLight"),
        LevelEntity,
        PointLightBundle {
            point_light: PointLight {
                intensity: 500.0,
                range: 15.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, WALL_HEIGHT - 0.2, 0.0),
            ..default()
        },
    ));

    // Spawn a placeholder puzzle interactable at a fixed spot per level.
    // TODO: replace with per-level bespoke puzzle layouts.
    world.spawn((
        Name::new(format!("Puzzle (Lvl {})", def.number)),
        LevelEntity,
        PuzzleInteractable {
            level: def.number,
            solved: false,
            prompt: format!("[E] Interact — Level {} puzzle", def.number),
        },
        Transform::from_xyz(2.0, 1.2, 2.0),
        Visibility::Visible,
    ));

    info!("Loaded level {}: {}", def.number, def.name);
}

/// Despawns all `LevelEntity`-tagged entities. Called on level exit.
pub fn unload_level(mut commands: Commands, query: Query<Entity, With<LevelEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("Unloaded level entities");
}

// --- Per-level enter/exit systems ---
// These are thin wrappers so we can register them in OnEnter/OnExit schedules.

pub fn enter_level1(world: &mut World) {
    load_level(GameState::Level1, world);
}
pub fn enter_level2(world: &mut World) {
    load_level(GameState::Level2, world);
}
pub fn enter_level3(world: &mut World) {
    load_level(GameState::Level3, world);
}
pub fn enter_level4(world: &mut World) {
    load_level(GameState::Level4, world);
}
pub fn enter_level5(world: &mut World) {
    load_level(GameState::Level5, world);
}
pub fn enter_level6(world: &mut World) {
    load_level(GameState::Level6, world);
}
pub fn enter_level7(world: &mut World) {
    load_level(GameState::Level7, world);
}
