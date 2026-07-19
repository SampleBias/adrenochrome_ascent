//! Stationary security turrets (TODO-020).

use bevy::prelude::*;

use adrenochrome_engine::{cast_ray, Billboard, MapGrid};

use crate::combat::{CombatTarget, HitFlash};
use crate::floor_loader::FloorEntity;
use crate::player::{apply_damage, Armor, Health, PainFlash, Player, PlayerMotor};

use super::ai::has_line_of_sight;
use super::archetype::{TEX_TURRET, TEX_TURRET_FIRE};
use super::components::Turret;

pub fn spawn_turret(commands: &mut Commands, pos: Vec2, yaw: f32, scale: f32) -> Entity {
    commands
        .spawn((
            FloorEntity,
            Name::new("Turret"),
            Turret {
                facing: yaw,
                view_range: 9.0,
                view_fov_cos: 0.55,
                fire_cooldown: 0.0,
                damage: 6.0,
                idle_texture: TEX_TURRET,
                fire_texture: TEX_TURRET_FIRE,
            },
            CombatTarget {
                health: 25.0,
                max_health: 25.0,
                dead: false,
            },
            HitFlash::default(),
            Billboard::new(pos, TEX_TURRET, scale),
            Transform::from_xyz(pos.x, pos.y, 0.0),
        ))
        .id()
}

/// Turrets track / fire hitscan at the player when they have LOS.
pub fn update_turrets(
    time: Res<Time>,
    map: Res<MapGrid>,
    mut pain: ResMut<PainFlash>,
    mut player: Query<(&PlayerMotor, &mut Health, &mut Armor), With<Player>>,
    mut turrets: Query<(&mut Turret, &mut Billboard, &CombatTarget)>,
) {
    let Ok((motor, mut health, mut armor)) = player.single_mut() else {
        return;
    };
    let dt = time.delta_secs();
    let player_pos = motor.pos;

    for (mut turret, mut billboard, target) in &mut turrets {
        if target.dead {
            continue;
        }
        turret.fire_cooldown = (turret.fire_cooldown - dt).max(0.0);
        billboard.texture_id = turret.idle_texture;

        let to = player_pos - billboard.pos;
        let dist = to.length();
        if dist > turret.view_range || dist < 0.05 {
            continue;
        }
        let dir = to / dist;
        let (s, c) = turret.facing.sin_cos();
        let forward = Vec2::new(c, s);
        if forward.dot(dir) < turret.view_fov_cos {
            continue;
        }
        if !has_line_of_sight(&map, billboard.pos, player_pos) {
            continue;
        }

        // Face the player while tracking.
        turret.facing = dir.y.atan2(dir.x);

        if turret.fire_cooldown > 0.0 {
            continue;
        }

        // Hitscan: wall must not block.
        let (wall_dist, _, _, _) = cast_ray(&map, billboard.pos, dir);
        if wall_dist + 0.1 < dist {
            continue;
        }

        apply_damage(&mut health, &mut armor, turret.damage);
        pain.trigger(0.25);
        turret.fire_cooldown = 0.7;
        billboard.texture_id = turret.fire_texture;
    }
}
