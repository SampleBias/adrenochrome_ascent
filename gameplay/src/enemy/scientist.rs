//! Mad Scientist boss — Floor 10 (TODO-025).

use bevy::prelude::*;

use adrenochrome_engine::Billboard;

use crate::combat::CombatTarget;
use crate::player::{apply_serum, Armor, Health, PainFlash, Player, PlayerMotor, SerumEffect};
use crate::puzzle::{DnaSequencer, PuzzleRegistry};

use super::archetype::TEX_SCIENTIST_ATK;
use super::components::{EnemyAi, EnemyState};

/// Global Scientist encounter state.
#[derive(Resource, Debug, Clone)]
pub struct ScientistFight {
    pub active: bool,
    pub phase: u8,
    pub defeated: bool,
    pub arena_center: Vec2,
    pub teleport_points: Vec<Vec2>,
}

impl Default for ScientistFight {
    fn default() -> Self {
        Self {
            active: false,
            phase: 0,
            defeated: false,
            arena_center: Vec2::new(10.5, 6.5),
            teleport_points: vec![
                Vec2::new(8.5, 5.5),
                Vec2::new(12.5, 5.5),
                Vec2::new(10.5, 7.5),
                Vec2::new(7.5, 7.5),
                Vec2::new(13.5, 7.5),
            ],
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct ScientistBoss {
    pub phase: u8,
    pub teleport_cooldown: f32,
    pub serum_cooldown: f32,
    pub stun_timer: f32,
    pub dna_requested: bool,
}

impl Default for ScientistBoss {
    fn default() -> Self {
        Self {
            phase: 0,
            teleport_cooldown: 2.0,
            serum_cooldown: 3.0,
            stun_timer: 0.0,
            dna_requested: false,
        }
    }
}

pub fn detect_scientist_presence(
    mut fight: ResMut<ScientistFight>,
    query: Query<&ScientistBoss>,
) {
    let present = !query.is_empty();
    if present && !fight.active && !fight.defeated {
        fight.active = true;
        fight.phase = 1;
        info!("Scientist boss fight engaged");
    }
    if !present && fight.active && !fight.defeated {
        fight.active = false;
        fight.defeated = true;
    }
}

pub fn tick_scientist_fight(
    time: Res<Time>,
    mut fight: ResMut<ScientistFight>,
    mut registry: ResMut<PuzzleRegistry>,
    mut dna: ResMut<DnaSequencer>,
    mut serum: ResMut<SerumEffect>,
    mut pain: ResMut<PainFlash>,
    player: Query<&PlayerMotor, With<Player>>,
    mut health_q: Query<(&mut Health, &mut Armor), With<Player>>,
    mut boss_q: Query<(
        &mut ScientistBoss,
        &mut EnemyAi,
        &mut Billboard,
        &mut Transform,
        &CombatTarget,
    )>,
) {
    if !fight.active || fight.defeated {
        return;
    }
    let dt = time.delta_secs();
    let Ok((mut boss, mut ai, mut billboard, mut transform, target)) = boss_q.single_mut() else {
        return;
    };

    if target.dead {
        fight.defeated = true;
        fight.active = false;
        registry.set("scientist_down", true);
        dna.active = false;
        info!("Scientist defeated");
        return;
    }

    fight.phase = boss.phase.max(1);

    // Mid-fight DNA phases at HP thresholds.
    let ratio = target.health / target.max_health;
    if boss.phase == 0 && ratio < 0.75 {
        boss.phase = 1;
        fight.phase = 1;
        boss.dna_requested = true;
        boss.stun_timer = 20.0;
        ai.state = EnemyState::Stunned;
        dna.start("scientist_dna_1", "ACGT");
        info!("Scientist demands DNA sequence ACGT");
    } else if boss.phase == 1 && registry.get("dna_sequence_correct") && boss.dna_requested {
        boss.dna_requested = false;
        boss.stun_timer = 0.0;
        registry.set("dna_sequence_correct", false);
        boss.phase = 2;
        fight.phase = 2;
        ai.state = EnemyState::Chase;
    } else if boss.phase == 2 && ratio < 0.4 && !boss.dna_requested {
        boss.phase = 3;
        fight.phase = 3;
        boss.dna_requested = true;
        boss.stun_timer = 20.0;
        ai.state = EnemyState::Stunned;
        dna.start("scientist_dna_2", "TGCA");
        info!("Scientist demands DNA sequence TGCA");
    } else if boss.phase == 3 && registry.get("dna_sequence_correct") && boss.dna_requested {
        boss.dna_requested = false;
        boss.stun_timer = 0.0;
        registry.set("dna_sequence_correct", false);
        boss.phase = 4;
        fight.phase = 4;
        ai.state = EnemyState::Chase;
    }

    if boss.stun_timer > 0.0 {
        boss.stun_timer -= dt;
        billboard.texture_id = TEX_SCIENTIST_ATK;
        return;
    }

    // Teleport around the arena.
    boss.teleport_cooldown -= dt;
    if boss.teleport_cooldown <= 0.0 && !fight.teleport_points.is_empty() {
        let idx = (time.elapsed_secs() as usize) % fight.teleport_points.len();
        let dest = fight.teleport_points[idx];
        billboard.pos = dest;
        transform.translation.x = dest.x;
        transform.translation.y = dest.y;
        boss.teleport_cooldown = 3.5 - (boss.phase as f32 * 0.3).min(1.5);
        billboard.texture_id = TEX_SCIENTIST_ATK;
        info!("Scientist teleported to {dest:?}");
    }

    // Serum attack: apply debuff when player is in mid range.
    boss.serum_cooldown -= dt;
    if let Ok(motor) = player.single() {
        let dist = billboard.pos.distance(motor.pos);
        if boss.serum_cooldown <= 0.0 && dist < 4.5 && dist > 1.2 {
            apply_serum(&mut serum, 7.0);
            if let Ok((mut health, mut armor)) = health_q.single_mut() {
                crate::player::apply_damage(&mut health, &mut armor, 6.0);
            }
            pain.trigger(0.3);
            boss.serum_cooldown = 4.0;
            billboard.texture_id = TEX_SCIENTIST_ATK;
            info!("Scientist serum attack");
        }
    }

    if ai.state == EnemyState::Attack {
        billboard.texture_id = TEX_SCIENTIST_ATK;
    } else {
        billboard.texture_id = ai.idle_texture;
    }
}

pub fn reset_scientist_fight(mut fight: ResMut<ScientistFight>, mut dna: ResMut<DnaSequencer>) {
    *fight = ScientistFight::default();
    *dna = DnaSequencer::default();
}
