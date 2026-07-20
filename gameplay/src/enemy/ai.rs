//! Grunt AI state machine: Patrol → Chase → Attack (TODO-018).

use bevy::prelude::*;

use adrenochrome_engine::{cast_ray, Billboard, MapGrid};

use crate::combat::{CombatTarget, HitFlash};
use crate::player::{
    apply_damage, apply_serum, Armor, Health, PainFlash, Player, PlayerMotor, SerumEffect,
};

use super::components::{Enemy, EnemyAi, EnemyArchetype, EnemyState, PlayerDetected};
use super::boss::LieutenantBoss;
use super::turret::spawn_turret;
use super::scientist::ScientistBoss;
use super::warden::WardenBoss;

const ENEMY_RADIUS_DEFAULT: f32 = 0.22;

/// True if a wall does not occlude `from` → `to`.
pub fn has_line_of_sight(map: &MapGrid, from: Vec2, to: Vec2) -> bool {
    let delta = to - from;
    let dist = delta.length();
    if dist < 0.05 {
        return true;
    }
    let dir = delta / dist;
    let (wall_dist, _, _, _) = cast_ray(map, from, dir);
    wall_dist >= dist - 0.15
}

fn can_see_player(map: &MapGrid, ai: &EnemyAi, pos: Vec2, player: Vec2) -> bool {
    let to = player - pos;
    let dist = to.length();
    if dist > ai.view_range || dist < 0.01 {
        return false;
    }
    let dir = to / dist;
    let (s, c) = ai.facing.sin_cos();
    let forward = Vec2::new(c, s);
    if forward.dot(dir) < ai.view_fov_cos {
        return false;
    }
    has_line_of_sight(map, pos, player)
}

/// Drive Patrol / Chase / Attack / Stunned for all non-dead enemies.
pub fn update_enemy_ai(
    time: Res<Time>,
    map: Res<MapGrid>,
    mut detected: MessageWriter<PlayerDetected>,
    player_q: Query<&PlayerMotor, With<Player>>,
    mut enemies: Query<(
        Entity,
        &Enemy,
        &mut EnemyAi,
        &mut Billboard,
        &mut Transform,
        &CombatTarget,
        Option<&mut LieutenantBoss>,
        Option<&mut WardenBoss>,
        Option<&mut ScientistBoss>,
    )>,
) {
    let Ok(player) = player_q.single() else {
        return;
    };
    let dt = time.delta_secs();
    let player_pos = player.pos;

    for (
        entity,
        enemy,
        mut ai,
        mut billboard,
        mut transform,
        target,
        mut lt_boss,
        mut warden,
        mut scientist,
    ) in &mut enemies
    {
        if target.dead || ai.state == EnemyState::Dead {
            ai.state = EnemyState::Dead;
            continue;
        }

        let mut stunned = false;
        if let Some(ref mut boss) = lt_boss {
            if boss.stun_timer > 0.0 {
                boss.stun_timer -= dt;
                stunned = true;
            }
        }
        if let Some(ref mut boss) = warden {
            if boss.stun_timer > 0.0 {
                boss.stun_timer = (boss.stun_timer - dt).max(0.0);
                if boss.stun_timer > 0.0 {
                    stunned = true;
                }
            }
        }
        if let Some(ref mut boss) = scientist {
            if boss.stun_timer > 0.0 {
                boss.stun_timer = (boss.stun_timer - dt).max(0.0);
                if boss.stun_timer > 0.0 {
                    stunned = true;
                }
            }
        }
        if stunned {
            ai.state = EnemyState::Stunned;
            billboard.texture_id = ai.attack_texture;
            transform.translation.x = billboard.pos.x;
            transform.translation.y = billboard.pos.y;
            continue;
        }

        ai.attack_cooldown = (ai.attack_cooldown - dt).max(0.0);
        let pos = billboard.pos;
        let sees = can_see_player(&map, &ai, pos, player_pos);
        let dist = pos.distance(player_pos);

        match ai.state {
            EnemyState::Patrol => {
                billboard.texture_id = ai.idle_texture;
                if sees {
                    ai.lose_sight_timer = 0.0;
                    detected.write(PlayerDetected { enemy: entity });
                    ai.state = if ai.flees {
                        EnemyState::Flee
                    } else {
                        EnemyState::Chase
                    };
                } else {
                    patrol_step(&map, &mut ai, &mut billboard, dt);
                }
            }
            EnemyState::Chase => {
                billboard.texture_id = ai.idle_texture;
                if sees {
                    ai.lose_sight_timer = 0.0;
                } else {
                    ai.lose_sight_timer += dt;
                    if ai.lose_sight_timer > 2.5 {
                        ai.state = EnemyState::Patrol;
                        continue;
                    }
                }
                if dist <= ai.attack_range {
                    ai.state = EnemyState::Attack;
                } else {
                    let chase = ai.chase_speed;
                    move_toward(&map, &mut ai, &mut billboard, player_pos, chase, dt);
                }
            }
            EnemyState::Flee => {
                billboard.texture_id = ai.attack_texture;
                if !sees {
                    ai.lose_sight_timer += dt;
                    if ai.lose_sight_timer > 3.0 {
                        ai.state = EnemyState::Patrol;
                        continue;
                    }
                } else {
                    ai.lose_sight_timer = 0.0;
                }
                let away = pos + (pos - player_pos).normalize_or_zero() * 3.0;
                let flee_speed = ai.chase_speed;
                move_toward(&map, &mut ai, &mut billboard, away, flee_speed, dt);
            }
            EnemyState::Attack => {
                billboard.texture_id = ai.attack_texture;
                face_toward(&mut ai, pos, player_pos);
                if dist > ai.attack_range * 1.35 {
                    ai.state = if ai.flees {
                        EnemyState::Flee
                    } else {
                        EnemyState::Chase
                    };
                } else if !sees && dist > ai.attack_range {
                    ai.state = EnemyState::Chase;
                }
            }
            EnemyState::Stunned => {
                billboard.texture_id = ai.attack_texture;
                let lt_ok = lt_boss.as_ref().map(|b| b.stun_timer <= 0.0).unwrap_or(true);
                let w_ok = warden.as_ref().map(|b| b.stun_timer <= 0.0).unwrap_or(true);
                let s_ok = scientist
                    .as_ref()
                    .map(|b| b.stun_timer <= 0.0)
                    .unwrap_or(true);
                if lt_ok && w_ok && s_ok {
                    ai.state = EnemyState::Chase;
                }
            }
            EnemyState::Dead => {}
        }

        // Erratic jitter for Zed / Mutated Aide while chasing.
        if matches!(
            enemy.archetype,
            EnemyArchetype::Zed | EnemyArchetype::MutatedAide
        ) && ai.state == EnemyState::Chase
        {
            let t = time.elapsed_secs() * 9.0 + entity.to_bits() as f32 * 0.01;
            let jitter = Vec2::new(t.sin(), t.cos()) * 0.35 * dt;
            billboard.pos = map.try_move(billboard.pos, jitter, ai.radius);
        }

        transform.translation.x = billboard.pos.x;
        transform.translation.y = billboard.pos.y;
        let _ = ENEMY_RADIUS_DEFAULT;
    }
}

fn face_toward(ai: &mut EnemyAi, from: Vec2, to: Vec2) {
    let d = to - from;
    if d.length_squared() > 1e-6 {
        ai.facing = d.y.atan2(d.x);
    }
}

fn patrol_step(map: &MapGrid, ai: &mut EnemyAi, billboard: &mut Billboard, dt: f32) {
    if ai.waypoints.len() < 2 {
        return;
    }
    let target = ai.waypoints[ai.waypoint_idx % ai.waypoints.len()];
    let dist = billboard.pos.distance(target);
    if dist < 0.15 {
        ai.waypoint_idx = (ai.waypoint_idx + 1) % ai.waypoints.len();
        return;
    }
    move_toward(map, ai, billboard, target, ai.speed, dt);
}

fn move_toward(
    map: &MapGrid,
    ai: &mut EnemyAi,
    billboard: &mut Billboard,
    target: Vec2,
    speed: f32,
    dt: f32,
) {
    let delta = target - billboard.pos;
    let dist = delta.length();
    if dist < 1e-4 {
        return;
    }
    face_toward(ai, billboard.pos, target);
    let step = delta.normalize() * speed * dt;
    billboard.pos = map.try_move(billboard.pos, step, ai.radius);
}

/// Melee damage when in Attack state and cooldown ready.
pub fn enemy_melee_attack(
    time: Res<Time>,
    mut pain: ResMut<PainFlash>,
    mut serum: ResMut<SerumEffect>,
    mut player: Query<(&PlayerMotor, &mut Health, &mut Armor), With<Player>>,
    mut enemies: Query<(&mut EnemyAi, &Billboard, &CombatTarget)>,
) {
    let Ok((motor, mut health, mut armor)) = player.single_mut() else {
        return;
    };
    let _ = time;
    for (mut ai, billboard, target) in &mut enemies {
        if target.dead || ai.state != EnemyState::Attack {
            continue;
        }
        if ai.attack_cooldown > 0.0 {
            continue;
        }
        let dist = billboard.pos.distance(motor.pos);
        if dist > ai.attack_range {
            continue;
        }
        apply_damage(&mut health, &mut armor, ai.attack_damage);
        pain.trigger((ai.attack_damage / 40.0).clamp(0.15, 0.7));
        if ai.applies_serum {
            apply_serum(&mut serum, 8.0);
        }
        ai.attack_cooldown = match ai.attack_range {
            r if r > 1.3 => 1.2,
            r if r < 1.05 => 0.55,
            _ => 0.85,
        };
    }
}

/// Sync CombatTarget death into AI state; flash bosses on hit.
pub fn sync_enemy_death_state(
    mut query: Query<(&CombatTarget, &mut EnemyAi, Option<&HitFlash>)>,
) {
    for (target, mut ai, _) in &mut query {
        if target.dead {
            ai.state = EnemyState::Dead;
        }
    }
}

/// Patrol Security radio: alert nearby same-faction allies to Chase.
pub fn radio_alert_allies(
    mut events: MessageReader<PlayerDetected>,
    detectors: Query<(Entity, &Enemy, &EnemyAi, &Billboard)>,
    mut allies: Query<(Entity, &Enemy, &mut EnemyAi, &Billboard)>,
) {
    const RADIO_RANGE: f32 = 8.0;
    for ev in events.read() {
        let Ok((_e, det_enemy, det_ai, det_bb)) = detectors.get(ev.enemy) else {
            continue;
        };
        if !det_ai.radio_alert {
            continue;
        }
        for (ally_e, ally_enemy, mut ally_ai, ally_bb) in &mut allies {
            if ally_e == ev.enemy {
                continue;
            }
            if ally_enemy.faction != det_enemy.faction {
                continue;
            }
            if det_bb.pos.distance(ally_bb.pos) > RADIO_RANGE {
                continue;
            }
            if matches!(
                ally_ai.state,
                EnemyState::Patrol | EnemyState::Stunned | EnemyState::Flee
            ) {
                ally_ai.state = if ally_ai.flees {
                    EnemyState::Flee
                } else {
                    EnemyState::Chase
                };
                ally_ai.lose_sight_timer = 0.0;
            }
        }
    }
}

/// Hazard Techs drop a turret the first time they chase.
pub fn deploy_tech_turrets(
    mut commands: Commands,
    mut techs: Query<(&Enemy, &mut EnemyAi, &Billboard)>,
) {
    for (enemy, mut ai, billboard) in &mut techs {
        if enemy.archetype != EnemyArchetype::HazardTech {
            continue;
        }
        if !ai.deploys_turret || ai.turret_deployed {
            continue;
        }
        if !matches!(ai.state, EnemyState::Chase | EnemyState::Attack) {
            continue;
        }
        ai.turret_deployed = true;
        let (s, c) = ai.facing.sin_cos();
        let behind = billboard.pos - Vec2::new(c, s) * 0.8;
        spawn_turret(&mut commands, behind, ai.facing, 0.55);
        info!("Hazard Tech deployed a turret");
    }
}

/// True if a shielded enemy is facing the shot origin (frontal block).
pub fn shield_blocks_shot(ai: &EnemyAi, target_pos: Vec2, shot_origin: Vec2) -> bool {
    if !ai.has_shield {
        return false;
    }
    let incoming = (shot_origin - target_pos).normalize_or_zero();
    let (s, c) = ai.facing.sin_cos();
    let forward = Vec2::new(c, s);
    forward.dot(incoming) > 0.3
}

#[cfg(test)]
mod tests {
    use super::*;
    use adrenochrome_engine::MapGrid;

    #[test]
    fn los_clear_in_open_cell() {
        let map = MapGrid::from_rows(&["#####", "#...#", "#...#", "#...#", "#####"]);
        assert!(has_line_of_sight(
            &map,
            Vec2::new(1.5, 1.5),
            Vec2::new(3.5, 3.5)
        ));
    }

    #[test]
    fn los_blocked_by_wall() {
        let map = MapGrid::from_rows(&["#####", "#.#.#", "#####"]);
        assert!(!has_line_of_sight(
            &map,
            Vec2::new(1.5, 1.5),
            Vec2::new(3.5, 1.5)
        ));
    }
}
