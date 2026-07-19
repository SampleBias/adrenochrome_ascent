//! Spawn helpers for floor loader + boss waves.

use bevy::prelude::*;

use adrenochrome_engine::Billboard;

use crate::combat::{CombatTarget, HitFlash};
use crate::floor_loader::FloorEntity;

use super::archetype::{archetype_stats, make_ai};
use super::boss::LieutenantBoss;
use super::components::{Enemy, EnemyArchetype, Faction};
use super::warden::WardenBoss;

/// Spawn a fully wired enemy entity at `pos`.
pub fn spawn_enemy(
    commands: &mut Commands,
    pos: Vec2,
    faction: Faction,
    archetype: EnemyArchetype,
    scale_override: f32,
    waypoints: Vec<Vec2>,
    yaw: f32,
) -> Entity {
    let stats = archetype_stats(archetype);
    let scale = if scale_override > 0.0 {
        scale_override
    } else {
        stats.scale
    };
    let ai = make_ai(archetype, waypoints, yaw);
    let mut entity = commands.spawn((
        FloorEntity,
        Name::new(format!("Enemy:{archetype:?}")),
        Enemy { faction, archetype },
        ai,
        CombatTarget {
            health: stats.health,
            max_health: stats.health,
            dead: false,
        },
        HitFlash::default(),
        Billboard::new(pos, stats.idle_texture, scale),
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));
    match archetype {
        EnemyArchetype::Lieutenant => {
            entity.insert(LieutenantBoss::default());
        }
        EnemyArchetype::Warden => {
            entity.insert(WardenBoss::default());
        }
        _ => {}
    }
    entity.id()
}
