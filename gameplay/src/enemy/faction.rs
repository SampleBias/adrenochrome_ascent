//! Global faction defeat / spare tracking + floor cleanup (TODO-023 / TODO-031).

use std::collections::HashSet;

use bevy::prelude::*;

use crate::combat::CombatTarget;
use crate::puzzle::PuzzleRegistry;

use super::components::{Enemy, Faction, Turret};

/// Tracks which factions have been defeated or spared this run.
#[derive(Resource, Debug, Clone, Default)]
pub struct FactionRegistry {
    pub defeated: HashSet<Faction>,
    pub spared: HashSet<Faction>,
}

impl FactionRegistry {
    pub fn is_defeated(&self, faction: Faction) -> bool {
        self.defeated.contains(&faction)
    }

    pub fn is_spared(&self, faction: Faction) -> bool {
        self.spared.contains(&faction)
    }

    pub fn mark_defeated(&mut self, faction: Faction) {
        self.defeated.insert(faction);
        self.spared.remove(&faction);
    }

    pub fn mark_spared(&mut self, faction: Faction) {
        self.spared.insert(faction);
        self.defeated.remove(&faction);
    }

    pub fn clear(&mut self) {
        self.defeated.clear();
        self.spared.clear();
    }

    pub fn spare_flag(faction: Faction) -> &'static str {
        match faction {
            Faction::Mob => "spare_mob",
            Faction::Security => "spare_security",
            Faction::Research => "spare_research",
            Faction::Executive => "spare_executive",
        }
    }

    pub fn defeat_flag(faction: Faction) -> &'static str {
        match faction {
            Faction::Mob => "faction_mob_defeated",
            Faction::Security => "faction_security_defeated",
            Faction::Research => "faction_research_defeated",
            Faction::Executive => "faction_executive_defeated",
        }
    }
}

/// Detect boss archetype deaths and trigger faction cleanup or spare.
pub fn watch_boss_defeats(
    mut commands: Commands,
    mut factions: ResMut<FactionRegistry>,
    mut puzzles: ResMut<PuzzleRegistry>,
    enemies: Query<(Entity, &Enemy, &CombatTarget)>,
    turrets: Query<Entity, With<Turret>>,
) {
    let mut defeat: Option<Faction> = None;
    for (_entity, enemy, target) in &enemies {
        if target.dead
            && enemy.archetype.is_boss()
            && !factions.is_defeated(enemy.faction)
            && !factions.is_spared(enemy.faction)
        {
            defeat = Some(enemy.faction);
            break;
        }
    }
    let Some(faction) = defeat else {
        return;
    };

    let spare = puzzles.get(FactionRegistry::spare_flag(faction));
    if spare {
        factions.mark_spared(faction);
        let spared_flag = match faction {
            Faction::Mob => "faction_mob_spared",
            Faction::Security => "faction_security_spared",
            Faction::Research => "faction_research_spared",
            Faction::Executive => "faction_executive_spared",
        };
        puzzles.set(spared_flag, true);
        puzzles.add_counter("moral_score", 1);
        info!("Faction {faction:?} spared — allies stand down");
        return;
    }

    factions.mark_defeated(faction);
    puzzles.set(FactionRegistry::defeat_flag(faction), true);
    info!("Faction {faction:?} defeated — clearing floor allies");

    for (entity, enemy, target) in &enemies {
        if enemy.faction == faction && !enemy.archetype.is_boss() && !target.dead {
            commands.entity(entity).despawn();
        }
    }
    if faction == Faction::Security {
        for entity in &turrets {
            commands.entity(entity).despawn();
        }
    }
}

/// Skip spawning enemies for already-defeated factions.
pub fn should_spawn_faction(factions: &FactionRegistry, faction: Faction) -> bool {
    !factions.is_defeated(faction)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marks_and_queries_defeat() {
        let mut reg = FactionRegistry::default();
        assert!(!reg.is_defeated(Faction::Mob));
        reg.mark_defeated(Faction::Mob);
        assert!(reg.is_defeated(Faction::Mob));
        assert!(!reg.is_defeated(Faction::Security));
    }

    #[test]
    fn spare_overrides_defeat() {
        let mut reg = FactionRegistry::default();
        reg.mark_defeated(Faction::Mob);
        reg.mark_spared(Faction::Mob);
        assert!(reg.is_spared(Faction::Mob));
        assert!(!reg.is_defeated(Faction::Mob));
    }
}
