//! Load RON floors into MapGrid + spawn floor-scoped entities (TODO-007).

use bevy::prelude::*;

use adrenochrome_content::{load_floor_number, EntityKind};
use adrenochrome_engine::{ActivePalette, Billboard, HandOverlay, MapGrid, RayCamera};

use crate::combat::{CombatTarget, HitFlash};
use crate::enemy::{
    should_spawn_faction, spawn_enemy, spawn_turret, BossFight, EnemyArchetype, Faction,
    FactionRegistry, ScientistFight, WardenOverrides, TEX_LIMB,
};
use crate::hazard::{spawn_crate, TimedValveState};
use crate::interact::{Interactable, LimbPickup};
use crate::puzzle::DnaSequencer;
use crate::player::{Player, PlayerMotor};
use crate::puzzle::PuzzleRegistry;

/// Marker on entities that belong to the currently loaded floor (despawned on unload).
#[derive(Component, Debug, Clone, Copy)]
pub struct FloorEntity;

/// Metadata for the loaded floor (HUD / elevators).
#[derive(Resource, Debug, Clone, Default)]
pub struct LoadedFloorInfo {
    pub number: u8,
    pub name: String,
    pub subtitle: String,
    pub ambient_audio: String,
    pub intro_text: String,
}

/// Startup: placeholder map + camera + persistent hand overlay until a floor loads.
pub fn setup_world_shell(mut commands: Commands) {
    commands.insert_resource(MapGrid::from_rows(&["###", "#.#", "###"]));
    commands.insert_resource(RayCamera::from_yaw(Vec2::new(1.5, 1.5), 0.0));
    commands.insert_resource(LoadedFloorInfo::default());
    commands.spawn((Name::new("HandOverlay"), HandOverlay::default()));
}

/// Load the floor matching [`crate::game::CurrentFloor`] when entering InGame.
pub fn load_current_floor(
    mut commands: Commands,
    floor: Res<crate::game::CurrentFloor>,
    mut map: ResMut<MapGrid>,
    mut camera: ResMut<RayCamera>,
    mut palette: ResMut<ActivePalette>,
    mut info: ResMut<LoadedFloorInfo>,
    mut boss: ResMut<BossFight>,
    mut warden: ResMut<WardenOverrides>,
    mut scientist: ResMut<ScientistFight>,
    mut dna: ResMut<DnaSequencer>,
    mut valves: ResMut<TimedValveState>,
    factions: Res<FactionRegistry>,
    floor_entities: Query<Entity, With<FloorEntity>>,
    mut players: Query<&mut PlayerMotor, With<Player>>,
) {
    let def = match load_floor_number(floor.number) {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to load floor {}: {e}", floor.number);
            return;
        }
    };

    for entity in &floor_entities {
        commands.entity(entity).despawn();
    }
    *boss = BossFight::default();
    *warden = WardenOverrides::default();
    *scientist = ScientistFight::default();
    *dna = DnaSequencer::default();
    valves.active.clear();
    if def.number == 3 {
        boss.arena_center = Vec2::new(10.5, 7.5);
    }
    if def.number == 7 {
        warden.arena_center = Vec2::new(10.5, 7.5);
    }
    if def.number == 10 {
        scientist.arena_center = Vec2::new(10.5, 6.5);
    }

    *map = def.to_map_grid();
    let spawn = Vec2::new(def.player_spawn.0, def.player_spawn.1);
    *camera = RayCamera::from_yaw(spawn, def.player_yaw);
    palette.palette = def.palette.into();
    *info = LoadedFloorInfo {
        number: def.number,
        name: def.name.clone(),
        subtitle: def.subtitle.clone(),
        ambient_audio: def.ambient_audio.clone(),
        intro_text: def.intro_text.clone(),
    };

    for mut motor in &mut players {
        motor.pos = spawn;
        motor.yaw = def.player_yaw;
        motor.velocity = Vec2::ZERO;
        motor.pitch = 0.0;
    }

    for ent in &def.entities {
        let pos = Vec2::new(ent.pos.0, ent.pos.1);
        match &ent.kind {
            EntityKind::Billboard { texture_id, scale } => {
                let mut entity = commands.spawn((
                    FloorEntity,
                    Name::new("FloorBillboard"),
                    Billboard::new(pos, *texture_id, *scale),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ));
                if *texture_id == 0 {
                    entity.insert((CombatTarget::default(), HitFlash::default()));
                }
            }
            EntityKind::Enemy {
                faction,
                archetype,
                scale,
                waypoints,
                yaw,
            } => {
                let faction = Faction::from(*faction);
                if !should_spawn_faction(&factions, faction) {
                    continue;
                }
                let wps: Vec<Vec2> = waypoints.iter().map(|p| Vec2::new(p.0, p.1)).collect();
                spawn_enemy(
                    &mut commands,
                    pos,
                    faction,
                    EnemyArchetype::from(*archetype),
                    *scale,
                    wps,
                    *yaw,
                );
            }
            EntityKind::Crate { scale } => {
                spawn_crate(&mut commands, &mut map, pos, *scale);
            }
            EntityKind::Turret { yaw, scale } => {
                spawn_turret(&mut commands, pos, *yaw, *scale);
            }
            EntityKind::Limb { amount, scale } => {
                commands.spawn((
                    FloorEntity,
                    Name::new("LimbPickup"),
                    LimbPickup { amount: *amount },
                    Billboard::new(pos, TEX_LIMB, *scale),
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ));
            }
            EntityKind::Interactable {
                prompt,
                require,
                action,
                texture_id,
                scale,
            } => {
                let mut entity = commands.spawn((
                    FloorEntity,
                    Name::new(prompt.clone()),
                    Interactable {
                        prompt: prompt.clone(),
                        require: require.clone(),
                        action: action.clone(),
                    },
                    Transform::from_xyz(pos.x, pos.y, 0.0),
                ));
                if let Some(tid) = texture_id {
                    entity.insert(Billboard::new(pos, *tid, *scale));
                }
            }
        }
    }

    info!(
        "Loaded floor {} — {} | audio cue '{}'",
        def.number, def.name, def.ambient_audio
    );
}

/// Clear floor entities + puzzle flags when returning to the main menu.
pub fn unload_floor(
    mut commands: Commands,
    floor_entities: Query<Entity, With<FloorEntity>>,
    mut registry: ResMut<PuzzleRegistry>,
    mut boss: ResMut<BossFight>,
    mut warden: ResMut<WardenOverrides>,
    mut scientist: ResMut<ScientistFight>,
    mut dna: ResMut<DnaSequencer>,
    mut valves: ResMut<TimedValveState>,
    mut factions: ResMut<FactionRegistry>,
) {
    for entity in &floor_entities {
        commands.entity(entity).despawn();
    }
    registry.clear();
    *boss = BossFight::default();
    *warden = WardenOverrides::default();
    *scientist = ScientistFight::default();
    *dna = DnaSequencer::default();
    valves.active.clear();
    factions.clear();
}
