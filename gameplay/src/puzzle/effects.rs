//! Puzzle DSL effect application (TODO-026 / TODO-031).

use bevy::prelude::*;

use adrenochrome_content::PuzzleEffectId;
use adrenochrome_engine::MapGrid;

use crate::combat::CombatTarget;
use crate::enemy::{Enemy, EnemyArchetype, FloorAlarm, ScientistFight};

use super::PuzzleRegistry;

/// Apply a list of puzzle effects to the world.
pub fn apply_effects(
    effects: &[PuzzleEffectId],
    registry: &mut PuzzleRegistry,
    map: &mut MapGrid,
    scientist_fight: &mut ScientistFight,
    alarm: &mut FloorAlarm,
    bosses: &mut Query<(&Enemy, &mut CombatTarget)>,
) {
    for effect in effects {
        apply_one(effect, registry, map, scientist_fight, alarm, bosses);
    }
}

fn apply_one(
    effect: &PuzzleEffectId,
    registry: &mut PuzzleRegistry,
    map: &mut MapGrid,
    scientist_fight: &mut ScientistFight,
    alarm: &mut FloorAlarm,
    bosses: &mut Query<(&Enemy, &mut CombatTarget)>,
) {
    match effect {
        PuzzleEffectId::SetFlag(flag) => {
            registry.set(flag.clone(), true);
            info!("DSL SetFlag({flag})");
        }
        PuzzleEffectId::ClearFlag(flag) => {
            registry.set(flag.clone(), false);
            info!("DSL ClearFlag({flag})");
        }
        PuzzleEffectId::OpenDoor { cell, flag } => {
            open_door_cells(map, cell.0, cell.1);
            if let Some(flag) = flag {
                registry.set(flag.clone(), true);
            }
            info!("DSL OpenDoor({},{})", cell.0, cell.1);
        }
        PuzzleEffectId::AddCounter { name, amount } => {
            registry.add_counter(name.clone(), *amount);
            info!(
                "DSL AddCounter({name}, {amount}) → {}",
                registry.counter(name)
            );
        }
        PuzzleEffectId::SetCounter { name, value } => {
            registry.set_counter(name.clone(), *value);
            info!("DSL SetCounter({name}, {value})");
        }
        PuzzleEffectId::DamageBoss(amount) => {
            for (enemy, mut target) in bosses.iter_mut() {
                if enemy.archetype == EnemyArchetype::Scientist && !target.dead {
                    target.health = (target.health - *amount).max(0.0);
                    if target.health <= 0.0 {
                        target.dead = true;
                    }
                    info!("DSL DamageBoss({amount}) → HP {}", target.health);
                }
            }
        }
        PuzzleEffectId::AdvanceBossPhase => {
            scientist_fight.phase = scientist_fight.phase.saturating_add(1);
            info!("DSL AdvanceBossPhase → {}", scientist_fight.phase);
        }
        PuzzleEffectId::MoralBump { flag, score } => {
            registry.set(flag.clone(), true);
            registry.add_counter("moral_score", *score);
            info!("DSL MoralBump({flag}, {score})");
        }
        PuzzleEffectId::TriggerAlarm => {
            alarm.raise(45.0);
            registry.set("floor_alarm", true);
            info!("DSL TriggerAlarm");
        }
    }
}

fn open_door_cells(map: &mut MapGrid, x: i32, y: i32) {
    for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
        let cx = x + dx;
        let cy = y + dy;
        if cx < 0 || cy < 0 {
            continue;
        }
        if map.get(cx as isize, cy as isize) != 0 {
            map.set(cx as usize, cy as usize, 0);
        }
    }
}
