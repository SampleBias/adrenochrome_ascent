//! Warden boss fight — Floor 7 (TODO-021).

use bevy::prelude::*;

use adrenochrome_engine::Billboard;

use crate::combat::CombatTarget;
use crate::player::{apply_damage, Armor, Health, PainFlash, Player, PlayerMotor};
use crate::puzzle::PuzzleRegistry;

use super::archetype::{archetype_stats, TEX_WARDEN_ATK};
use super::components::{Enemy, EnemyAi, EnemyArchetype, EnemyState, Faction};
use super::spawn::spawn_enemy;

/// Boss-driven overrides for the hazard / flood systems during Floor 7.
#[derive(Resource, Debug, Clone)]
pub struct WardenOverrides {
    pub active: bool,
    pub phase: u8,
    pub flood_level: f32,
    pub combat_paused: bool,
    pub force_flood: bool,
    pub defeated: bool,
    pub arena_center: Vec2,
    pub valve_prompt_sent: bool,
}

impl Default for WardenOverrides {
    fn default() -> Self {
        Self {
            active: false,
            phase: 0,
            flood_level: 0.0,
            combat_paused: false,
            force_flood: false,
            defeated: false,
            arena_center: Vec2::new(10.5, 7.5),
            valve_prompt_sent: false,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct WardenBoss {
    pub phase: u8,
    pub stun_timer: f32,
    pub summon_cooldown: f32,
    pub hp_phase_threshold: f32,
}

impl Default for WardenBoss {
    fn default() -> Self {
        Self {
            phase: 0,
            stun_timer: 0.0,
            summon_cooldown: 3.0,
            hp_phase_threshold: 0.66,
        }
    }
}

pub fn detect_warden_presence(
    mut overrides: ResMut<WardenOverrides>,
    query: Query<&WardenBoss>,
) {
    let present = !query.is_empty();
    if present && !overrides.active && !overrides.defeated {
        overrides.active = true;
        overrides.phase = 1;
        overrides.flood_level = 0.0;
        overrides.combat_paused = false;
        info!("Warden boss fight engaged");
    }
    if !present && overrides.active && !overrides.defeated {
        overrides.active = false;
        overrides.defeated = true;
    }
}

pub fn tick_warden_fight(
    time: Res<Time>,
    mut commands: Commands,
    mut overrides: ResMut<WardenOverrides>,
    mut registry: ResMut<PuzzleRegistry>,
    mut boss_q: Query<(
        &mut WardenBoss,
        &mut EnemyAi,
        &mut Billboard,
        &CombatTarget,
    )>,
    grunts: Query<Entity, (With<Enemy>, Without<WardenBoss>)>,
) {
    if !overrides.active || overrides.defeated {
        return;
    }
    let dt = time.delta_secs();

    let Ok((mut boss, mut ai, mut billboard, target)) = boss_q.single_mut() else {
        return;
    };

    if target.dead {
        overrides.defeated = true;
        overrides.active = false;
        overrides.combat_paused = false;
        overrides.force_flood = false;
        registry.set("warden_down", true);
        info!("Warden defeated");
        return;
    }

    // Mid-fight valve pauses: enter pause when HP crosses thresholds.
    let ratio = target.health / target.max_health;
    if boss.phase == 0 && ratio < 0.66 {
        boss.phase = 1;
        overrides.phase = 1;
        overrides.combat_paused = true;
        overrides.force_flood = true;
        overrides.valve_prompt_sent = false;
        registry.set("warden_valve_a", false);
        ai.state = EnemyState::Stunned;
        boss.stun_timer = 999.0;
        info!("Warden phase 1 — open coolant valve A");
    } else if boss.phase == 1 && registry.get("warden_valve_a") {
        overrides.combat_paused = false;
        overrides.force_flood = false;
        boss.stun_timer = 0.0;
        ai.state = EnemyState::Chase;
        boss.phase = 2;
        overrides.phase = 2;
        boss.hp_phase_threshold = 0.33;
        info!("Valve A cleared — combat resumes");
    } else if boss.phase == 2 && ratio < 0.33 {
        boss.phase = 3;
        overrides.phase = 3;
        overrides.combat_paused = true;
        overrides.force_flood = true;
        registry.set("warden_valve_b", false);
        ai.state = EnemyState::Stunned;
        boss.stun_timer = 999.0;
        info!("Warden phase 3 — open drain valve B");
    } else if boss.phase == 3 && registry.get("warden_valve_b") {
        overrides.combat_paused = false;
        overrides.force_flood = false;
        boss.stun_timer = 0.0;
        ai.state = EnemyState::Chase;
        boss.phase = 4;
        overrides.phase = 4;
        info!("Valve B cleared — final phase");
    }

    if overrides.force_flood {
        overrides.flood_level = (overrides.flood_level + dt * 0.12).min(0.9);
    } else if overrides.flood_level > 0.0 {
        overrides.flood_level = (overrides.flood_level - dt * 0.08).max(0.0);
    }

    if overrides.combat_paused {
        billboard.texture_id = TEX_WARDEN_ATK;
        // Freeze nearby grunts by not summoning; boss stays stunned.
        return;
    }

    boss.summon_cooldown -= dt;
    let grunt_count = grunts.iter().count();
    if boss.summon_cooldown <= 0.0 && grunt_count < 3 {
        spawn_security_wave(&mut commands, overrides.arena_center, overrides.phase);
        boss.summon_cooldown = 9.0;
    }

    if ai.state == EnemyState::Attack {
        billboard.texture_id = TEX_WARDEN_ATK;
    } else {
        billboard.texture_id = ai.idle_texture;
    }
}

fn spawn_security_wave(commands: &mut Commands, center: Vec2, phase: u8) {
    let offsets = [Vec2::new(-2.0, 1.5), Vec2::new(2.0, 1.5), Vec2::new(0.0, -2.2)];
    let count = (phase.max(1)).min(3) as usize;
    for (i, off) in offsets.iter().take(count).enumerate() {
        let archetype = match i % 3 {
            0 => EnemyArchetype::PatrolSecurity,
            1 => EnemyArchetype::RiotGuard,
            _ => EnemyArchetype::HazardTech,
        };
        spawn_enemy(
            commands,
            center + *off,
            Faction::Security,
            archetype,
            archetype_stats(archetype).scale,
            vec![],
            0.0,
        );
    }
}

/// Flood damage driven by WardenOverrides (and shared basin logic).
pub fn apply_warden_flood(
    time: Res<Time>,
    overrides: Res<WardenOverrides>,
    mut pain: ResMut<PainFlash>,
    mut player: Query<(&PlayerMotor, &mut Health, &mut Armor), With<Player>>,
) {
    if !overrides.active || overrides.flood_level < 0.12 {
        return;
    }
    let Ok((motor, mut health, mut armor)) = player.single_mut() else {
        return;
    };
    if motor.pos.distance(overrides.arena_center) > 6.0 {
        return;
    }
    let dps = 5.0 + overrides.flood_level * 14.0;
    apply_damage(&mut health, &mut armor, dps * time.delta_secs());
    if (time.elapsed_secs() * 3.0) as i32 % 2 == 0 {
        pain.trigger(0.14 * overrides.flood_level);
    }
}

/// While combat is paused, keep the Warden stunned and non-aggressive.
pub fn enforce_warden_pause(
    overrides: Res<WardenOverrides>,
    mut boss_q: Query<(&mut EnemyAi, &mut WardenBoss)>,
) {
    if !overrides.combat_paused {
        return;
    }
    for (mut ai, mut boss) in &mut boss_q {
        ai.state = EnemyState::Stunned;
        boss.stun_timer = 1.0;
        ai.attack_cooldown = 1.0;
    }
}

pub fn reset_warden_overrides(mut overrides: ResMut<WardenOverrides>) {
    *overrides = WardenOverrides::default();
}
