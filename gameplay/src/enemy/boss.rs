//! Lieutenant boss fight — Floor 3 (TODO-017).

use bevy::prelude::*;

use adrenochrome_engine::Billboard;

use crate::combat::{CombatTarget, HitFlash};
use crate::floor_loader::ActiveWaveTuning;
use crate::player::{apply_damage, Armor, Health, PainFlash, Player, PlayerMotor};
use crate::puzzle::PuzzleRegistry;

use super::archetype::{archetype_stats, TEX_LIEUTENANT_ATK};
use super::components::{Enemy, EnemyAi, EnemyArchetype, EnemyState, Faction};
use super::spawn::spawn_enemy;

/// Global boss encounter state for Floor 3.
#[derive(Resource, Debug, Clone)]
pub struct BossFight {
    pub active: bool,
    pub phase: u8,
    pub wave_timer: f32,
    pub flood_level: f32,
    pub cigar_timer: f32,
    pub cigar_vulnerable: bool,
    pub defeated: bool,
    pub arena_center: Vec2,
}

impl Default for BossFight {
    fn default() -> Self {
        Self {
            active: false,
            phase: 0,
            wave_timer: 0.0,
            flood_level: 0.0,
            cigar_timer: 0.0,
            cigar_vulnerable: false,
            defeated: false,
            arena_center: Vec2::new(10.5, 7.5),
        }
    }
}

/// Marker on the Lieutenant entity.
#[derive(Component, Debug, Clone)]
pub struct LieutenantBoss {
    pub phase: u8,
    pub stun_timer: f32,
    pub summon_cooldown: f32,
    pub hits_on_cigar: u32,
}

impl Default for LieutenantBoss {
    fn default() -> Self {
        Self {
            phase: 0,
            stun_timer: 0.0,
            summon_cooldown: 2.0,
            hits_on_cigar: 0,
        }
    }
}

/// Activate boss systems when a Lieutenant is present on the floor.
pub fn detect_boss_presence(
    mut fight: ResMut<BossFight>,
    query: Query<&LieutenantBoss>,
) {
    let present = !query.is_empty();
    if present && !fight.active && !fight.defeated {
        fight.active = true;
        fight.phase = 1;
        fight.wave_timer = 1.5;
        fight.flood_level = 0.0;
        info!("Lieutenant boss fight engaged");
    }
    if !present && fight.active && !fight.defeated {
        fight.active = false;
        fight.defeated = true;
    }
}

/// Cycle cigar vulnerable windows, rise flood, summon waves.
pub fn tick_boss_fight(
    time: Res<Time>,
    mut commands: Commands,
    mut fight: ResMut<BossFight>,
    mut registry: ResMut<PuzzleRegistry>,
    waves: Res<ActiveWaveTuning>,
    mut boss_q: Query<(
        &mut LieutenantBoss,
        &mut EnemyAi,
        &mut Billboard,
        &CombatTarget,
    )>,
    grunts: Query<Entity, (With<Enemy>, Without<LieutenantBoss>)>,
) {
    if !fight.active || fight.defeated {
        return;
    }
    let dt = time.delta_secs();

    // Cigar weakpoint cadence: vulnerable ~1.2s every 4s.
    fight.cigar_timer += dt;
    if fight.cigar_timer > 4.0 {
        fight.cigar_timer = 0.0;
    }
    fight.cigar_vulnerable = fight.cigar_timer < 1.2;

    // Flood rises with phase.
    let target_flood = (fight.phase as f32 * 0.22).min(0.85);
    fight.flood_level = (fight.flood_level + dt * 0.04).min(target_flood);

    let Ok((mut boss, mut ai, mut billboard, target)) = boss_q.single_mut() else {
        return;
    };
    if target.dead {
        fight.defeated = true;
        fight.active = false;
        registry.set("lieutenant_down", true);
        info!("Lieutenant defeated");
        return;
    }

    if fight.cigar_vulnerable {
        billboard.texture_id = TEX_LIEUTENANT_ATK;
    } else if ai.state != EnemyState::Attack && ai.state != EnemyState::Stunned {
        billboard.texture_id = ai.idle_texture;
    }

    boss.summon_cooldown -= dt;
    fight.wave_timer -= dt;

    let grunt_count = grunts.iter().count();
    let max_grunts = waves.max_grunts as usize;
    if boss.summon_cooldown <= 0.0 && grunt_count < max_grunts && fight.phase < 4 {
        let wave = fight.phase.max(1);
        spawn_wave(&mut commands, fight.arena_center, wave, waves.max_grunts);
        boss.summon_cooldown = (waves.cooldown_secs - wave as f32 * 0.5).max(4.0);
        fight.wave_timer = 6.0;
        info!("Lieutenant summoned wave {wave}");
    }

    if boss.hits_on_cigar >= 2 && boss.phase < 3 {
        boss.phase += 1;
        fight.phase = boss.phase;
        boss.hits_on_cigar = 0;
        boss.stun_timer = 1.5;
        ai.state = EnemyState::Stunned;
        info!("Lieutenant phase → {}", boss.phase);
    }
}

fn spawn_wave(commands: &mut Commands, center: Vec2, wave: u8, max_grunts: u8) {
    let offsets = [
        Vec2::new(-2.5, 1.5),
        Vec2::new(2.5, 1.5),
        Vec2::new(0.0, -2.0),
        Vec2::new(-1.5, -1.5),
    ];
    let count = wave.clamp(1, max_grunts.max(1)) as usize;
    for (i, off) in offsets.iter().take(count).enumerate() {
        let archetype = match (wave + i as u8) % 3 {
            0 => EnemyArchetype::Thug,
            1 => EnemyArchetype::Zed,
            _ => EnemyArchetype::Heavy,
        };
        let pos = center + *off;
        spawn_enemy(
            commands,
            pos,
            Faction::Mob,
            archetype,
            archetype_stats(archetype).scale,
            vec![],
            0.0,
        );
    }
}

/// Flood water damages the player when standing in the arena basin.
pub fn apply_flood_hazard(
    time: Res<Time>,
    fight: Res<BossFight>,
    mut pain: ResMut<PainFlash>,
    mut player: Query<(&PlayerMotor, &mut Health, &mut Armor), With<Player>>,
) {
    if !fight.active || fight.flood_level < 0.15 {
        return;
    }
    let Ok((motor, mut health, mut armor)) = player.single_mut() else {
        return;
    };
    let in_basin = motor.pos.distance(fight.arena_center) < 5.5;
    if !in_basin {
        return;
    }
    let dps = 4.0 + fight.flood_level * 12.0;
    apply_damage(&mut health, &mut armor, dps * time.delta_secs());
    if (time.elapsed_secs() * 3.0) as i32 % 2 == 0 {
        pain.trigger(0.12 * fight.flood_level);
    }
}

/// Called from hitscan when a Lieutenant takes a hit.
pub fn register_lieutenant_hit(
    fight: &BossFight,
    boss: &mut LieutenantBoss,
    target: &mut CombatTarget,
    ai: &mut EnemyAi,
    flash: &mut HitFlash,
    base_damage: f32,
) {
    if !fight.cigar_vulnerable {
        // Body shots are heavily resisted outside the cigar window.
        target.health = (target.health + base_damage * 0.65).min(target.max_health);
        return;
    }
    // Weakpoint burst during cigar flare.
    let bonus = base_damage * 2.5;
    target.health -= bonus;
    boss.hits_on_cigar += 1;
    boss.stun_timer = 0.8;
    ai.state = EnemyState::Stunned;
    flash.timer = 0.25;
    if target.health <= 0.0 {
        target.health = 0.0;
        target.dead = true;
    }
}

/// Clear boss fight when leaving the floor / menu.
pub fn reset_boss_fight(mut fight: ResMut<BossFight>) {
    *fight = BossFight::default();
}
