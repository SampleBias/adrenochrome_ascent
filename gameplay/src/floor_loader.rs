//! Load RON floors into MapGrid + spawn floor-scoped entities (TODO-007).

use bevy::prelude::*;

use adrenochrome_content::{load_floor_asset, load_floor_number, EntityKind, WaveTuning};
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

/// Active floor wave budget for boss summons (TODO-036).
#[derive(Resource, Debug, Clone, Copy)]
pub struct ActiveWaveTuning {
    pub max_grunts: u8,
    pub cooldown_secs: f32,
}

impl Default for ActiveWaveTuning {
    fn default() -> Self {
        let w = WaveTuning::default();
        Self {
            max_grunts: w.max_grunts,
            cooldown_secs: w.cooldown_secs,
        }
    }
}

impl From<WaveTuning> for ActiveWaveTuning {
    fn from(w: WaveTuning) -> Self {
        Self {
            max_grunts: w.max_grunts,
            cooldown_secs: w.cooldown_secs,
        }
    }
}

/// One-shot skip for Options→InGame soft resume (TODO-040).
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct SkipFloorLoad(pub bool);

/// Consume soft-resume into a one-frame floor-load skip.
pub fn begin_ingame_enter(
    mut soft_resume: ResMut<crate::game::SoftInGameResume>,
    mut skip: ResMut<SkipFloorLoad>,
) {
    skip.0 = soft_resume.0;
    if soft_resume.0 {
        soft_resume.0 = false;
        info!("Soft resume — keeping loaded floor");
    }
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
    skip: Res<SkipFloorLoad>,
    mut map: ResMut<MapGrid>,
    mut camera: ResMut<RayCamera>,
    mut palette: ResMut<ActivePalette>,
    mut info: ResMut<LoadedFloorInfo>,
    mut boss: ResMut<BossFight>,
    mut warden: ResMut<WardenOverrides>,
    mut scientist: ResMut<ScientistFight>,
    mut dna: ResMut<DnaSequencer>,
    mut valves: ResMut<TimedValveState>,
    mut waves: ResMut<ActiveWaveTuning>,
    factions: Res<FactionRegistry>,
    floor_entities: Query<Entity, With<FloorEntity>>,
    mut players: Query<&mut PlayerMotor, With<Player>>,
) {
    if skip.0 {
        return;
    }

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
    *waves = def.resolved_wave_tuning().into();

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

/// Floor 9 softlock failsafe: local limb if uplink path is unreachable (TODO-038).
pub fn floor9_limb_failsafe(
    floor: Res<crate::game::CurrentFloor>,
    skip: Res<SkipFloorLoad>,
    mut commands: Commands,
    mut registry: ResMut<PuzzleRegistry>,
) {
    if skip.0 || floor.number != 9 {
        return;
    }
    if registry.counter("collected_limb") >= 1
        || registry.get("exec_open")
        || registry.get("f9_limb_failsafe")
    {
        return;
    }
    let pos = Vec2::new(10.5, 12.5);
    commands.spawn((
        FloorEntity,
        Name::new("LimbFailsafe"),
        LimbPickup { amount: 1 },
        Billboard::new(pos, TEX_LIMB, 0.55),
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));
    registry.set("f9_limb_failsafe", true);
    warn!("Floor 9 failsafe: spawned local limb near spawn");
}

/// Outdoor limo convoy scene for the ending cinematic (TODO-040).
pub fn load_ending_cinematic(
    mut commands: Commands,
    mut map: ResMut<MapGrid>,
    mut camera: ResMut<RayCamera>,
    mut palette: ResMut<ActivePalette>,
    mut info: ResMut<LoadedFloorInfo>,
    floor_entities: Query<Entity, With<FloorEntity>>,
) {
    let def = match load_floor_asset("floors/ending_road.ron") {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to load ending cinematic: {e}");
            return;
        }
    };

    for entity in &floor_entities {
        commands.entity(entity).despawn();
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

    // Always spawn convoy props — ignore faction defeat filters for the cinematic.
    for ent in &def.entities {
        let pos = Vec2::new(ent.pos.0, ent.pos.1);
        if let EntityKind::Enemy {
            faction,
            archetype,
            scale,
            waypoints,
            yaw,
        } = &ent.kind
        {
            let wps: Vec<Vec2> = waypoints.iter().map(|p| Vec2::new(p.0, p.1)).collect();
            spawn_enemy(
                &mut commands,
                pos,
                Faction::from(*faction),
                EnemyArchetype::from(*archetype),
                *scale,
                wps,
                *yaw,
            );
        }
    }

    info!("Ending cinematic loaded — {}", def.name);
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
