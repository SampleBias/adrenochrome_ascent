//! Archetype stats + sprite ids (TODO-016 / TODO-020).

use super::components::{EnemyAi, EnemyArchetype, EnemyState, Faction};

/// Procedural sprite indices (see `engine::textures`).
pub const TEX_THUG: usize = 12;
pub const TEX_THUG_ATK: usize = 13;
pub const TEX_HEAVY: usize = 14;
pub const TEX_HEAVY_ATK: usize = 15;
pub const TEX_ZED: usize = 16;
pub const TEX_ZED_ATK: usize = 17;
pub const TEX_LIEUTENANT: usize = 18;
pub const TEX_LIEUTENANT_ATK: usize = 19;
pub const TEX_LOOT_AMMO: usize = 20;
pub const TEX_LOOT_MEDKIT: usize = 21;
pub const TEX_RIOT: usize = 22;
pub const TEX_RIOT_ATK: usize = 23;
pub const TEX_PATROL: usize = 24;
pub const TEX_PATROL_ATK: usize = 25;
pub const TEX_TECH: usize = 26;
pub const TEX_TECH_ATK: usize = 27;
pub const TEX_WARDEN: usize = 28;
pub const TEX_WARDEN_ATK: usize = 29;
pub const TEX_CRATE: usize = 30;
pub const TEX_TURRET: usize = 31;
pub const TEX_TURRET_FIRE: usize = 32;

#[derive(Debug, Clone, Copy)]
pub struct ArchetypeStats {
    pub health: f32,
    pub scale: f32,
    pub speed: f32,
    pub chase_speed: f32,
    pub attack_range: f32,
    pub attack_damage: f32,
    pub attack_cooldown: f32,
    pub view_range: f32,
    pub view_fov_cos: f32,
    pub radius: f32,
    pub idle_texture: usize,
    pub attack_texture: usize,
    pub faction: Faction,
    pub has_shield: bool,
    pub radio_alert: bool,
    pub deploys_turret: bool,
}

pub fn archetype_stats(archetype: EnemyArchetype) -> ArchetypeStats {
    match archetype {
        EnemyArchetype::Thug => ArchetypeStats {
            health: 35.0,
            scale: 0.95,
            speed: 1.4,
            chase_speed: 2.4,
            attack_range: 1.15,
            attack_damage: 8.0,
            attack_cooldown: 0.85,
            view_range: 7.0,
            view_fov_cos: 0.45,
            radius: 0.22,
            idle_texture: TEX_THUG,
            attack_texture: TEX_THUG_ATK,
            faction: Faction::Mob,
            has_shield: false,
            radio_alert: false,
            deploys_turret: false,
        },
        EnemyArchetype::Heavy => ArchetypeStats {
            health: 90.0,
            scale: 1.15,
            speed: 0.9,
            chase_speed: 1.5,
            attack_range: 1.35,
            attack_damage: 18.0,
            attack_cooldown: 1.2,
            view_range: 6.0,
            view_fov_cos: 0.35,
            radius: 0.28,
            idle_texture: TEX_HEAVY,
            attack_texture: TEX_HEAVY_ATK,
            faction: Faction::Mob,
            has_shield: false,
            radio_alert: false,
            deploys_turret: false,
        },
        EnemyArchetype::Zed => ArchetypeStats {
            health: 22.0,
            scale: 0.85,
            speed: 1.8,
            chase_speed: 3.2,
            attack_range: 1.0,
            attack_damage: 6.0,
            attack_cooldown: 0.55,
            view_range: 8.5,
            view_fov_cos: 0.2,
            radius: 0.2,
            idle_texture: TEX_ZED,
            attack_texture: TEX_ZED_ATK,
            faction: Faction::Mob,
            has_shield: false,
            radio_alert: false,
            deploys_turret: false,
        },
        EnemyArchetype::Lieutenant => ArchetypeStats {
            health: 220.0,
            scale: 1.25,
            speed: 1.1,
            chase_speed: 1.8,
            attack_range: 1.5,
            attack_damage: 14.0,
            attack_cooldown: 1.0,
            view_range: 12.0,
            view_fov_cos: -0.2,
            radius: 0.3,
            idle_texture: TEX_LIEUTENANT,
            attack_texture: TEX_LIEUTENANT_ATK,
            faction: Faction::Mob,
            has_shield: false,
            radio_alert: false,
            deploys_turret: false,
        },
        EnemyArchetype::RiotGuard => ArchetypeStats {
            health: 70.0,
            scale: 1.1,
            speed: 0.85,
            chase_speed: 1.4,
            attack_range: 1.25,
            attack_damage: 12.0,
            attack_cooldown: 1.0,
            view_range: 6.5,
            view_fov_cos: 0.4,
            radius: 0.28,
            idle_texture: TEX_RIOT,
            attack_texture: TEX_RIOT_ATK,
            faction: Faction::Security,
            has_shield: true,
            radio_alert: false,
            deploys_turret: false,
        },
        EnemyArchetype::PatrolSecurity => ArchetypeStats {
            health: 45.0,
            scale: 0.95,
            speed: 1.5,
            chase_speed: 2.5,
            attack_range: 1.15,
            attack_damage: 9.0,
            attack_cooldown: 0.75,
            view_range: 8.0,
            view_fov_cos: 0.35,
            radius: 0.22,
            idle_texture: TEX_PATROL,
            attack_texture: TEX_PATROL_ATK,
            faction: Faction::Security,
            has_shield: false,
            radio_alert: true,
            deploys_turret: false,
        },
        EnemyArchetype::HazardTech => ArchetypeStats {
            health: 40.0,
            scale: 0.9,
            speed: 1.3,
            chase_speed: 2.0,
            attack_range: 1.1,
            attack_damage: 7.0,
            attack_cooldown: 0.9,
            view_range: 7.0,
            view_fov_cos: 0.4,
            radius: 0.22,
            idle_texture: TEX_TECH,
            attack_texture: TEX_TECH_ATK,
            faction: Faction::Security,
            has_shield: false,
            radio_alert: false,
            deploys_turret: true,
        },
        EnemyArchetype::Warden => ArchetypeStats {
            health: 280.0,
            scale: 1.3,
            speed: 1.0,
            chase_speed: 1.7,
            attack_range: 1.55,
            attack_damage: 16.0,
            attack_cooldown: 0.95,
            view_range: 14.0,
            view_fov_cos: -0.15,
            radius: 0.32,
            idle_texture: TEX_WARDEN,
            attack_texture: TEX_WARDEN_ATK,
            faction: Faction::Security,
            has_shield: true,
            radio_alert: true,
            deploys_turret: false,
        },
    }
}

pub fn make_ai(archetype: EnemyArchetype, waypoints: Vec<bevy::math::Vec2>, yaw: f32) -> EnemyAi {
    let s = archetype_stats(archetype);
    EnemyAi {
        state: EnemyState::Patrol,
        waypoints,
        waypoint_idx: 0,
        facing: yaw,
        attack_cooldown: 0.0,
        lose_sight_timer: 0.0,
        speed: s.speed,
        chase_speed: s.chase_speed,
        attack_range: s.attack_range,
        attack_damage: s.attack_damage,
        view_range: s.view_range,
        view_fov_cos: s.view_fov_cos,
        radius: s.radius,
        idle_texture: s.idle_texture,
        attack_texture: s.attack_texture,
        has_shield: s.has_shield,
        radio_alert: s.radio_alert,
        deploys_turret: s.deploys_turret,
        turret_deployed: false,
    }
}
