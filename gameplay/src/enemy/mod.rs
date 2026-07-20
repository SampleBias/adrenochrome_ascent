//! Enemy factions, AI, loot, bosses (Sprint 4–7).

mod ai;
mod alarm;
mod archetype;
mod boss;
mod components;
mod faction;
mod loot;
mod scientist;
mod spawn;
mod turret;
mod warden;

pub use ai::{
    deploy_tech_turrets, enemy_melee_attack, has_line_of_sight, radio_alert_allies,
    shield_blocks_shot, sync_enemy_death_state, update_enemy_ai,
};
pub use alarm::{
    apply_floor_alarm, reset_floor_alarm, secretary_raise_alarm, tick_floor_alarm, FloorAlarm,
};
pub use archetype::{archetype_stats, make_ai, TEX_CRATE, TEX_LIMB};
pub use boss::{
    apply_flood_hazard, detect_boss_presence, register_lieutenant_hit, reset_boss_fight,
    tick_boss_fight, BossFight, LieutenantBoss,
};
pub use components::{
    Enemy, EnemyAi, EnemyArchetype, EnemyState, Faction, PlayerDetected, Turret,
};
pub use faction::{should_spawn_faction, watch_boss_defeats, FactionRegistry};
pub use loot::{collect_loot, process_enemy_deaths, roll_loot, LootKind, LootPickup};
pub use scientist::{
    detect_scientist_presence, reset_scientist_fight, tick_scientist_fight, ScientistBoss,
    ScientistFight,
};
pub use spawn::spawn_enemy;
pub use turret::{spawn_turret, update_turrets};
pub use warden::{
    apply_warden_flood, detect_warden_presence, enforce_warden_pause, reset_warden_overrides,
    tick_warden_fight, WardenBoss, WardenOverrides,
};
