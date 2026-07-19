//! Faction-tied loot drops (TODO-019).

use bevy::prelude::*;

use adrenochrome_engine::Billboard;

use crate::combat::CombatTarget;
use crate::floor_loader::FloorEntity;
use crate::player::{Armor, Health, Inventory, ItemKind, Player, PlayerMotor, WeaponId};
use crate::puzzle::PuzzleRegistry;

use super::archetype::{TEX_LOOT_AMMO, TEX_LOOT_MEDKIT};
use super::components::{Enemy, EnemyArchetype, Faction};

#[derive(Debug, Clone)]
pub enum LootKind {
    Ammo9mm(u32),
    Shells(u32),
    Cells(u32),
    Adreno(u32),
    Health(f32),
    Armor(f32),
    Weapon(WeaponId),
    KeyFlag(String),
}

#[derive(Component, Debug, Clone)]
pub struct LootPickup {
    pub kind: LootKind,
}

/// Roll faction-appropriate loot for a killed enemy.
pub fn roll_loot(faction: Faction, archetype: EnemyArchetype) -> Option<LootKind> {
    match archetype {
        EnemyArchetype::Lieutenant => return Some(LootKind::KeyFlag("lieutenant_down".into())),
        EnemyArchetype::Warden => return Some(LootKind::KeyFlag("warden_down".into())),
        EnemyArchetype::Heavy if matches!(faction, Faction::Mob) => {
            return Some(LootKind::Shells(2));
        }
        EnemyArchetype::RiotGuard => return Some(LootKind::Armor(20.0)),
        EnemyArchetype::HazardTech => return Some(LootKind::Cells(6)),
        _ => {}
    }

    match faction {
        Faction::Mob => match archetype {
            EnemyArchetype::Thug => Some(LootKind::Ammo9mm(6)),
            EnemyArchetype::Zed => Some(LootKind::Health(15.0)),
            EnemyArchetype::Heavy => Some(LootKind::Shells(2)),
            EnemyArchetype::Lieutenant => Some(LootKind::KeyFlag("lieutenant_down".into())),
            _ => Some(LootKind::Ammo9mm(4)),
        },
        Faction::Security => match archetype {
            EnemyArchetype::PatrolSecurity => Some(LootKind::Shells(3)),
            EnemyArchetype::Warden => Some(LootKind::KeyFlag("warden_down".into())),
            _ => Some(LootKind::Shells(2)),
        },
        Faction::Research => Some(LootKind::Cells(8)),
        Faction::Executive => Some(LootKind::Adreno(1)),
    }
}

pub fn spawn_loot_at(commands: &mut Commands, pos: Vec2, kind: LootKind) {
    let (tex, scale) = match &kind {
        LootKind::Health(_) | LootKind::Armor(_) => (TEX_LOOT_MEDKIT, 0.4),
        LootKind::KeyFlag(_) => (3usize, 0.4),
        _ => (TEX_LOOT_AMMO, 0.4),
    };
    commands.spawn((
        FloorEntity,
        Name::new("LootPickup"),
        LootPickup { kind },
        Billboard::new(pos, tex, scale),
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));
}

/// On enemy death: drop loot, then despawn.
pub fn process_enemy_deaths(
    mut commands: Commands,
    mut registry: ResMut<PuzzleRegistry>,
    query: Query<(Entity, &CombatTarget, &Enemy, &Billboard)>,
) {
    for (entity, target, enemy, billboard) in &query {
        if !target.dead {
            continue;
        }
        if let Some(loot) = roll_loot(enemy.faction, enemy.archetype) {
            if let LootKind::KeyFlag(flag) = &loot {
                registry.set(flag, true);
            }
            spawn_loot_at(&mut commands, billboard.pos, loot);
        }
        commands.entity(entity).despawn();
    }
}

/// Walk-over pickup.
pub fn collect_loot(
    mut commands: Commands,
    mut registry: ResMut<PuzzleRegistry>,
    mut player: Query<(&PlayerMotor, &mut Health, &mut Armor, &mut Inventory), With<Player>>,
    loot: Query<(Entity, &LootPickup, &Billboard)>,
) {
    let Ok((motor, mut health, mut armor, mut inv)) = player.single_mut() else {
        return;
    };
    for (entity, pickup, billboard) in &loot {
        if motor.pos.distance(billboard.pos) > 0.7 {
            continue;
        }
        apply_loot(
            &mut health,
            &mut armor,
            &mut inv,
            &mut registry,
            &pickup.kind,
        );
        commands.entity(entity).despawn();
    }
}

fn apply_loot(
    health: &mut Health,
    armor: &mut Armor,
    inv: &mut Inventory,
    registry: &mut PuzzleRegistry,
    kind: &LootKind,
) {
    match kind {
        LootKind::Ammo9mm(n) => inv.ammo_9mm += *n,
        LootKind::Shells(n) => {
            inv.ammo_shells += *n;
            inv.grant_weapon(WeaponId::Shotgun);
        }
        LootKind::Cells(n) => {
            inv.ammo_cells += *n;
            inv.grant_weapon(WeaponId::Plasma);
        }
        LootKind::Adreno(n) => {
            inv.ammo_adreno += *n;
            inv.grant_weapon(WeaponId::Injector);
        }
        LootKind::Health(n) => {
            health.current = (health.current + *n).min(health.max);
        }
        LootKind::Armor(n) => {
            armor.current = (armor.current + *n).min(armor.max);
        }
        LootKind::Weapon(id) => inv.grant_weapon(*id),
        LootKind::KeyFlag(flag) => {
            registry.set(flag, true);
            let has_card = inv
                .slots
                .iter()
                .any(|s| matches!(s, Some(ItemKind::Keycard)));
            if !has_card {
                if let Some(slot) = inv.slots.iter_mut().find(|s| s.is_none()) {
                    *slot = Some(ItemKind::Keycard);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mob_thug_drops_ammo() {
        match roll_loot(Faction::Mob, EnemyArchetype::Thug).unwrap() {
            LootKind::Ammo9mm(n) => assert!(n > 0),
            _ => panic!("expected ammo"),
        }
    }

    #[test]
    fn lieutenant_sets_key_flag_loot() {
        match roll_loot(Faction::Mob, EnemyArchetype::Lieutenant).unwrap() {
            LootKind::KeyFlag(f) => assert_eq!(f, "lieutenant_down"),
            _ => panic!("expected key flag"),
        }
    }
}
