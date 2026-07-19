//! Weapon definitions and fire logic (TODO-013 / TODO-014).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use adrenochrome_engine::{cast_ray, Billboard, MapGrid};

use super::vitals::{apply_damage, Armor, Health, Inventory, PainFlash};
use crate::combat::{CombatTarget, HitFlash};
use crate::enemy::{
    register_lieutenant_hit, shield_blocks_shot, BossFight, EnemyAi, LieutenantBoss,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponId {
    Pistol,
    Shotgun,
    Plasma,
    Injector,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmmoType {
    Bullet9mm,
    Shell,
    Cell,
    Adreno,
}

#[derive(Debug, Clone, Copy)]
pub struct WeaponStats {
    pub id: WeaponId,
    pub name: &'static str,
    pub damage: f32,
    pub pellets: u8,
    pub spread: f32,
    pub range: f32,
    pub cooldown: f32,
    pub ammo: AmmoType,
    pub ammo_per_shot: u32,
    pub hand_texture: usize,
    pub fire_texture: usize,
}

pub fn weapon_stats(id: WeaponId) -> WeaponStats {
    match id {
        WeaponId::Pistol => WeaponStats {
            id,
            name: "9mm Pistol",
            damage: 12.0,
            pellets: 1,
            spread: 0.0,
            range: 18.0,
            cooldown: 0.28,
            ammo: AmmoType::Bullet9mm,
            ammo_per_shot: 1,
            hand_texture: 4,
            fire_texture: 5,
        },
        WeaponId::Shotgun => WeaponStats {
            id,
            name: "Shotgun",
            damage: 8.0,
            pellets: 6,
            spread: 0.12,
            range: 8.0,
            cooldown: 0.75,
            ammo: AmmoType::Shell,
            ammo_per_shot: 1,
            hand_texture: 6,
            fire_texture: 7,
        },
        WeaponId::Plasma => WeaponStats {
            id,
            name: "Plasma Rifle",
            damage: 16.0,
            pellets: 1,
            spread: 0.02,
            range: 22.0,
            cooldown: 0.12,
            ammo: AmmoType::Cell,
            ammo_per_shot: 1,
            hand_texture: 8,
            fire_texture: 9,
        },
        WeaponId::Injector => WeaponStats {
            id,
            name: "Adrenochrome Injector",
            damage: 0.0,
            pellets: 0,
            spread: 0.0,
            range: 0.0,
            cooldown: 0.5,
            ammo: AmmoType::Adreno,
            ammo_per_shot: 1,
            hand_texture: 10,
            fire_texture: 11,
        },
    }
}

#[derive(Component, Debug, Clone)]
pub struct WeaponLoadout {
    pub current: WeaponId,
    pub cooldown: f32,
    pub fire_anim: f32,
}

impl Default for WeaponLoadout {
    fn default() -> Self {
        Self {
            current: WeaponId::Pistol,
            cooldown: 0.0,
            fire_anim: 0.0,
        }
    }
}

/// Adrenochrome vision: reveal + health drain (TODO-013 / TODO-027 hook).
#[derive(Resource, Debug, Clone, Copy)]
pub struct AdrenoVision {
    pub active: bool,
    pub time_left: f32,
    pub drain_per_sec: f32,
}

impl Default for AdrenoVision {
    fn default() -> Self {
        Self {
            active: false,
            time_left: 0.0,
            drain_per_sec: 8.0,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct ScreenShake {
    pub trauma: f32,
}

impl ScreenShake {
    pub fn add(&mut self, amount: f32) {
        self.trauma = (self.trauma + amount).clamp(0.0, 1.0);
    }
}

#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct MuzzleFlash {
    pub timer: f32,
}

pub fn tick_weapon_timers(
    time: Res<Time>,
    mut loadout: Query<&mut WeaponLoadout>,
    mut muzzle: ResMut<MuzzleFlash>,
    mut shake: ResMut<ScreenShake>,
) {
    let dt = time.delta_secs();
    for mut w in &mut loadout {
        w.cooldown = (w.cooldown - dt).max(0.0);
        w.fire_anim = (w.fire_anim - dt).max(0.0);
    }
    muzzle.timer = (muzzle.timer - dt).max(0.0);
    shake.trauma = (shake.trauma - dt * 2.5).max(0.0);
}

pub fn select_weapon(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut WeaponLoadout, &Inventory)>,
) {
    let Ok((mut loadout, inv)) = query.single_mut() else {
        return;
    };
    let pick = if keys.just_pressed(KeyCode::Digit1) {
        Some(WeaponId::Pistol)
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some(WeaponId::Shotgun)
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some(WeaponId::Plasma)
    } else if keys.just_pressed(KeyCode::Digit4) {
        Some(WeaponId::Injector)
    } else {
        None
    };
    if let Some(id) = pick {
        if inv.has_weapon(id) || id == WeaponId::Pistol {
            loadout.current = id;
        }
    }
}

pub fn fire_weapon(
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    map: Res<MapGrid>,
    mut vision: ResMut<AdrenoVision>,
    mut muzzle: ResMut<MuzzleFlash>,
    mut shake: ResMut<ScreenShake>,
    mut pain: ResMut<PainFlash>,
    fight: Res<BossFight>,
    mut player: Query<(&mut WeaponLoadout, &mut Inventory, &mut Health, &super::PlayerMotor)>,
    mut targets: Query<(
        Entity,
        &mut CombatTarget,
        &mut Billboard,
        Option<&mut HitFlash>,
        Option<&mut LieutenantBoss>,
        Option<&mut EnemyAi>,
    )>,
) {
    let wants = mouse.pressed(MouseButton::Left) || keys.pressed(KeyCode::ControlLeft);
    if !wants {
        return;
    }
    let Ok((mut loadout, mut inv, mut health, motor)) = player.single_mut() else {
        return;
    };
    if loadout.cooldown > 0.0 {
        return;
    }

    let stats = weapon_stats(loadout.current);
    if stats.id == WeaponId::Injector {
        if !inv.try_consume(stats.ammo, stats.ammo_per_shot) {
            return;
        }
        vision.active = true;
        vision.time_left = 6.0;
        loadout.cooldown = stats.cooldown;
        loadout.fire_anim = 0.25;
        muzzle.timer = 0.12;
        shake.add(0.15);
        pain.trigger(0.2);
        // Immediate drain tick feel.
        health.current = (health.current - 4.0).max(1.0);
        return;
    }

    if !inv.try_consume(stats.ammo, stats.ammo_per_shot) {
        return;
    }

    loadout.cooldown = stats.cooldown;
    loadout.fire_anim = 0.18;
    muzzle.timer = 0.08;
    shake.add(if stats.id == WeaponId::Shotgun {
        0.45
    } else {
        0.2
    });

    let origin = motor.pos;
    let base_yaw = motor.yaw;
    for i in 0..stats.pellets {
        let offset = if stats.pellets == 1 {
            0.0
        } else {
            (i as f32 - (stats.pellets as f32 - 1.0) * 0.5) * stats.spread
        };
        let yaw = base_yaw + offset;
        let (s, c) = yaw.sin_cos();
        let dir = Vec2::new(c, s);
        hitscan(
            &map,
            origin,
            dir,
            stats.range,
            stats.damage,
            &fight,
            &mut targets,
        );
    }
}

fn hitscan(
    map: &MapGrid,
    origin: Vec2,
    dir: Vec2,
    range: f32,
    damage: f32,
    fight: &BossFight,
    targets: &mut Query<(
        Entity,
        &mut CombatTarget,
        &mut Billboard,
        Option<&mut HitFlash>,
        Option<&mut LieutenantBoss>,
        Option<&mut EnemyAi>,
    )>,
) {
    let (wall_dist, _, _, _) = cast_ray(map, origin, dir);
    let max_dist = wall_dist.min(range);

    let mut best: Option<(f32, Entity)> = None;
    for (entity, target, billboard, _, _, _) in targets.iter() {
        if target.health <= 0.0 {
            continue;
        }
        let rel = billboard.pos - origin;
        let depth = rel.dot(dir);
        if depth <= 0.05 || depth > max_dist {
            continue;
        }
        let lateral = (rel - dir * depth).length();
        if lateral > 0.45 {
            continue;
        }
        if best.as_ref().map(|(d, _)| depth < *d).unwrap_or(true) {
            best = Some((depth, entity));
        }
    }

    let Some((_, entity)) = best else {
        return;
    };

    if let Ok((_, mut target, mut billboard, flash, boss, ai)) = targets.get_mut(entity) {
        if let Some(ref ai_ref) = ai {
            if shield_blocks_shot(ai_ref, billboard.pos, origin) {
                if let Some(mut flash) = flash {
                    flash.timer = 0.08;
                }
                // Frontal shield: no damage, slight knock sound via flash only.
                return;
            }
        }
        target.health -= damage;
        if let Some(mut flash) = flash {
            flash.timer = 0.15;
            if let (Some(mut boss), Some(mut ai)) = (boss, ai) {
                register_lieutenant_hit(
                    fight,
                    &mut boss,
                    &mut target,
                    &mut ai,
                    &mut flash,
                    damage,
                );
            }
        } else if let (Some(mut boss), Some(mut ai)) = (boss, ai) {
            let mut tmp = HitFlash { timer: 0.15 };
            register_lieutenant_hit(fight, &mut boss, &mut target, &mut ai, &mut tmp, damage);
        }
        let away = (billboard.pos - origin).normalize_or_zero() * 0.08;
        billboard.pos += away;
        if target.health <= 0.0 {
            target.health = 0.0;
            target.dead = true;
        }
    }
}

pub fn tick_adreno_vision(
    time: Res<Time>,
    mut vision: ResMut<AdrenoVision>,
    mut player: Query<(&mut Health, &mut Armor), With<super::Player>>,
    mut pain: ResMut<PainFlash>,
) {
    if !vision.active {
        return;
    }
    vision.time_left -= time.delta_secs();
    if vision.time_left <= 0.0 {
        vision.active = false;
        vision.time_left = 0.0;
        return;
    }
    let Ok((mut health, mut armor)) = player.single_mut() else {
        return;
    };
    let drain = vision.drain_per_sec * time.delta_secs();
    apply_damage(&mut health, &mut armor, drain * 0.35);
    // Prefer direct health drain for injector fantasy.
    health.current = (health.current - drain * 0.65).max(1.0);
    if (vision.time_left * 10.0) as i32 % 4 == 0 {
        pain.trigger(0.08);
    }
}

/// Grant mid/late weapons for playtest (debug): F5 shotgun, F6 plasma.
pub fn debug_grant_weapons(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Inventory>,
) {
    let Ok(mut inv) = query.single_mut() else {
        return;
    };
    if keys.just_pressed(KeyCode::F5) {
        inv.grant_weapon(WeaponId::Shotgun);
        inv.ammo_shells += 8;
    }
    if keys.just_pressed(KeyCode::F6) {
        inv.grant_weapon(WeaponId::Plasma);
        inv.ammo_cells += 40;
    }
    if keys.just_pressed(KeyCode::F7) {
        inv.grant_weapon(WeaponId::Injector);
        inv.ammo_adreno += 2;
    }
}
