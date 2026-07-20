//! Archetype stats + sprite ids (TODO-016 / TODO-020 / TODO-024).

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
pub const TEX_RES_M: usize = 33;
pub const TEX_RES_M_ATK: usize = 34;
pub const TEX_RES_F: usize = 35;
pub const TEX_RES_F_ATK: usize = 36;
pub const TEX_AIDE: usize = 37;
pub const TEX_AIDE_ATK: usize = 38;
pub const TEX_SERUM_Z: usize = 39;
pub const TEX_SERUM_Z_ATK: usize = 40;
pub const TEX_SCIENTIST: usize = 41;
pub const TEX_SCIENTIST_ATK: usize = 42;
pub const TEX_LIMB: usize = 43;
pub const TEX_BODYGUARD: usize = 44;
pub const TEX_BODYGUARD_ATK: usize = 45;
pub const TEX_SECRETARY: usize = 46;
pub const TEX_SECRETARY_ATK: usize = 47;
pub const TEX_LIMO: usize = 48;
pub const TEX_LIMO_ATK: usize = 49;
pub const TEX_CEO: usize = 50;
pub const TEX_CEO_ATK: usize = 51;

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
    pub applies_serum: bool,
    pub flees: bool,
    pub triggers_alarm: bool,
}

fn base(
    health: f32,
    scale: f32,
    speed: f32,
    chase: f32,
    atk_range: f32,
    atk_dmg: f32,
    atk_cd: f32,
    view: f32,
    fov: f32,
    radius: f32,
    idle: usize,
    attack: usize,
    faction: Faction,
) -> ArchetypeStats {
    ArchetypeStats {
        health,
        scale,
        speed,
        chase_speed: chase,
        attack_range: atk_range,
        attack_damage: atk_dmg,
        attack_cooldown: atk_cd,
        view_range: view,
        view_fov_cos: fov,
        radius,
        idle_texture: idle,
        attack_texture: attack,
        faction,
        has_shield: false,
        radio_alert: false,
        deploys_turret: false,
        applies_serum: false,
        flees: false,
        triggers_alarm: false,
    }
}

pub fn archetype_stats(archetype: EnemyArchetype) -> ArchetypeStats {
    match archetype {
        EnemyArchetype::Thug => base(
            35.0, 0.95, 1.4, 2.4, 1.15, 8.0, 0.85, 7.0, 0.45, 0.22, TEX_THUG, TEX_THUG_ATK,
            Faction::Mob,
        ),
        EnemyArchetype::Heavy => base(
            90.0, 1.15, 0.9, 1.5, 1.35, 18.0, 1.2, 6.0, 0.35, 0.28, TEX_HEAVY, TEX_HEAVY_ATK,
            Faction::Mob,
        ),
        EnemyArchetype::Zed => base(
            22.0, 0.85, 1.8, 3.2, 1.0, 6.0, 0.55, 8.5, 0.2, 0.2, TEX_ZED, TEX_ZED_ATK,
            Faction::Mob,
        ),
        EnemyArchetype::Lieutenant => base(
            220.0, 1.25, 1.1, 1.8, 1.5, 14.0, 1.0, 12.0, -0.2, 0.3, TEX_LIEUTENANT,
            TEX_LIEUTENANT_ATK, Faction::Mob,
        ),
        EnemyArchetype::RiotGuard => {
            let mut s = base(
                70.0, 1.1, 0.85, 1.4, 1.25, 12.0, 1.0, 6.5, 0.4, 0.28, TEX_RIOT, TEX_RIOT_ATK,
                Faction::Security,
            );
            s.has_shield = true;
            s
        }
        EnemyArchetype::PatrolSecurity => {
            let mut s = base(
                45.0, 0.95, 1.5, 2.5, 1.15, 9.0, 0.75, 8.0, 0.35, 0.22, TEX_PATROL, TEX_PATROL_ATK,
                Faction::Security,
            );
            s.radio_alert = true;
            s
        }
        EnemyArchetype::HazardTech => {
            let mut s = base(
                40.0, 0.9, 1.3, 2.0, 1.1, 7.0, 0.9, 7.0, 0.4, 0.22, TEX_TECH, TEX_TECH_ATK,
                Faction::Security,
            );
            s.deploys_turret = true;
            s
        }
        EnemyArchetype::Warden => {
            let mut s = base(
                280.0, 1.3, 1.0, 1.7, 1.55, 16.0, 0.95, 14.0, -0.15, 0.32, TEX_WARDEN,
                TEX_WARDEN_ATK, Faction::Security,
            );
            s.has_shield = true;
            s.radio_alert = true;
            s
        }
        EnemyArchetype::ResearcherMale => {
            let mut s = base(
                20.0, 0.9, 1.6, 2.8, 0.9, 4.0, 1.0, 7.5, 0.3, 0.2, TEX_RES_M, TEX_RES_M_ATK,
                Faction::Research,
            );
            s.flees = true;
            s.radio_alert = true;
            s
        }
        EnemyArchetype::ResearcherFemale => {
            let mut s = base(
                20.0, 0.9, 1.7, 2.9, 0.9, 4.0, 1.0, 7.5, 0.3, 0.2, TEX_RES_F, TEX_RES_F_ATK,
                Faction::Research,
            );
            s.flees = true;
            s.radio_alert = true;
            s
        }
        EnemyArchetype::MutatedAide => base(
            50.0, 0.95, 2.0, 3.4, 1.05, 11.0, 0.5, 8.0, 0.15, 0.22, TEX_AIDE, TEX_AIDE_ATK,
            Faction::Research,
        ),
        EnemyArchetype::SerumZombie => {
            let mut s = base(
                100.0, 1.1, 0.7, 1.2, 1.2, 10.0, 1.1, 5.5, 0.25, 0.28, TEX_SERUM_Z, TEX_SERUM_Z_ATK,
                Faction::Research,
            );
            s.applies_serum = true;
            s
        }
        EnemyArchetype::Scientist => {
            let mut s = base(
                300.0, 1.25, 1.2, 2.0, 1.6, 14.0, 0.9, 14.0, -0.2, 0.3, TEX_SCIENTIST,
                TEX_SCIENTIST_ATK, Faction::Research,
            );
            s.applies_serum = true;
            s.radio_alert = true;
            s
        }
        EnemyArchetype::Bodyguard => {
            let mut s = base(
                110.0, 1.15, 0.95, 1.6, 1.3, 16.0, 0.95, 7.0, 0.4, 0.28, TEX_BODYGUARD,
                TEX_BODYGUARD_ATK, Faction::Executive,
            );
            s.has_shield = true;
            s
        }
        EnemyArchetype::AdminSecretary => {
            let mut s = base(
                18.0, 0.85, 1.5, 2.6, 0.85, 3.0, 1.2, 9.0, 0.25, 0.18, TEX_SECRETARY,
                TEX_SECRETARY_ATK, Faction::Executive,
            );
            s.flees = true;
            s.triggers_alarm = true;
            s.radio_alert = true;
            s
        }
        EnemyArchetype::LimoDriver => base(
            55.0, 1.0, 1.4, 2.3, 1.15, 10.0, 0.8, 7.5, 0.35, 0.22, TEX_LIMO, TEX_LIMO_ATK,
            Faction::Executive,
        ),
        EnemyArchetype::Ceo => {
            let mut s = base(
                200.0, 1.2, 1.0, 1.5, 1.4, 12.0, 1.1, 12.0, -0.1, 0.28, TEX_CEO, TEX_CEO_ATK,
                Faction::Executive,
            );
            s.has_shield = true;
            s.radio_alert = true;
            s
        }
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
        applies_serum: s.applies_serum,
        flees: s.flees,
        triggers_alarm: s.triggers_alarm,
    }
}
