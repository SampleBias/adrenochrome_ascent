//! Enemy ECS components (TODO-015 / TODO-020 / TODO-024).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Faction tier — maps to the boss hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Serialize, Deserialize)]
pub enum Faction {
    Mob,
    Security,
    Research,
    Executive,
}

impl From<adrenochrome_content::FactionId> for Faction {
    fn from(value: adrenochrome_content::FactionId) -> Self {
        match value {
            adrenochrome_content::FactionId::Mob => Self::Mob,
            adrenochrome_content::FactionId::Security => Self::Security,
            adrenochrome_content::FactionId::Research => Self::Research,
            adrenochrome_content::FactionId::Executive => Self::Executive,
        }
    }
}

/// Mob + Security + Research archetypes (and their bosses).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Serialize, Deserialize)]
pub enum EnemyArchetype {
    Thug,
    Heavy,
    Zed,
    Lieutenant,
    RiotGuard,
    PatrolSecurity,
    HazardTech,
    Warden,
    ResearcherMale,
    ResearcherFemale,
    MutatedAide,
    SerumZombie,
    Scientist,
    Bodyguard,
    AdminSecretary,
    LimoDriver,
    Ceo,
}

impl From<adrenochrome_content::EnemyArchetypeId> for EnemyArchetype {
    fn from(value: adrenochrome_content::EnemyArchetypeId) -> Self {
        match value {
            adrenochrome_content::EnemyArchetypeId::Thug => Self::Thug,
            adrenochrome_content::EnemyArchetypeId::Heavy => Self::Heavy,
            adrenochrome_content::EnemyArchetypeId::Zed => Self::Zed,
            adrenochrome_content::EnemyArchetypeId::Lieutenant => Self::Lieutenant,
            adrenochrome_content::EnemyArchetypeId::RiotGuard => Self::RiotGuard,
            adrenochrome_content::EnemyArchetypeId::PatrolSecurity => Self::PatrolSecurity,
            adrenochrome_content::EnemyArchetypeId::HazardTech => Self::HazardTech,
            adrenochrome_content::EnemyArchetypeId::Warden => Self::Warden,
            adrenochrome_content::EnemyArchetypeId::ResearcherMale => Self::ResearcherMale,
            adrenochrome_content::EnemyArchetypeId::ResearcherFemale => Self::ResearcherFemale,
            adrenochrome_content::EnemyArchetypeId::MutatedAide => Self::MutatedAide,
            adrenochrome_content::EnemyArchetypeId::SerumZombie => Self::SerumZombie,
            adrenochrome_content::EnemyArchetypeId::Scientist => Self::Scientist,
            adrenochrome_content::EnemyArchetypeId::Bodyguard => Self::Bodyguard,
            adrenochrome_content::EnemyArchetypeId::AdminSecretary => Self::AdminSecretary,
            adrenochrome_content::EnemyArchetypeId::LimoDriver => Self::LimoDriver,
            adrenochrome_content::EnemyArchetypeId::Ceo => Self::Ceo,
        }
    }
}

impl EnemyArchetype {
    pub fn is_boss(self) -> bool {
        matches!(
            self,
            Self::Lieutenant | Self::Warden | Self::Scientist | Self::Ceo
        )
    }

    pub fn applies_serum(self) -> bool {
        matches!(self, Self::SerumZombie | Self::Scientist)
    }

    pub fn flees(self) -> bool {
        matches!(
            self,
            Self::ResearcherMale | Self::ResearcherFemale | Self::AdminSecretary
        )
    }

    pub fn triggers_alarm(self) -> bool {
        matches!(self, Self::AdminSecretary)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum EnemyState {
    Patrol,
    Chase,
    Attack,
    Flee,
    Stunned,
    Dead,
}

/// Marker + identity for all combat AI actors.
#[derive(Component, Debug, Clone, Copy)]
pub struct Enemy {
    pub faction: Faction,
    pub archetype: EnemyArchetype,
}

/// Grunt / boss AI runtime state.
#[derive(Component, Debug, Clone)]
pub struct EnemyAi {
    pub state: EnemyState,
    pub waypoints: Vec<Vec2>,
    pub waypoint_idx: usize,
    pub facing: f32,
    pub attack_cooldown: f32,
    pub lose_sight_timer: f32,
    pub speed: f32,
    pub chase_speed: f32,
    pub attack_range: f32,
    pub attack_damage: f32,
    pub view_range: f32,
    pub view_fov_cos: f32,
    pub radius: f32,
    pub idle_texture: usize,
    pub attack_texture: usize,
    pub has_shield: bool,
    pub radio_alert: bool,
    pub deploys_turret: bool,
    pub turret_deployed: bool,
    pub applies_serum: bool,
    pub flees: bool,
    /// Admin Secretaries raise a floor-wide stealth alarm on detect (TODO-028).
    pub triggers_alarm: bool,
}

/// Fired when an enemy first acquires LOS on the player.
#[derive(Message, Debug, Clone, Copy)]
pub struct PlayerDetected {
    pub enemy: Entity,
}

/// Stationary hitscan turret (TODO-020).
#[derive(Component, Debug, Clone)]
pub struct Turret {
    pub facing: f32,
    pub view_range: f32,
    pub view_fov_cos: f32,
    pub fire_cooldown: f32,
    pub damage: f32,
    pub idle_texture: usize,
    pub fire_texture: usize,
}
