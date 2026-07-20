//! Permanent mutation perks at floors 3 / 6 / 9 (TODO-030).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::CurrentFloor;

use super::vitals::Inventory;
use super::Player;

/// Run-persistent mutation upgrades granted every three floors.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MutationPerks {
    pub speed: bool,
    pub inventory: bool,
    pub night_vision: bool,
}

impl MutationPerks {
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn speed_multiplier(self) -> f32 {
        if self.speed {
            1.22
        } else {
            1.0
        }
    }
}

const EXTRA_INVENTORY_SLOTS: usize = 4;

/// Grant perks when the player reaches floors 3, 6, and 9.
pub fn grant_mutation_perks(
    floor: Res<CurrentFloor>,
    mut perks: ResMut<MutationPerks>,
    mut inventory: Query<&mut Inventory, With<Player>>,
) {
    if floor.number >= 3 && !perks.speed {
        perks.speed = true;
        info!("Mutation perk: speed");
    }
    if floor.number >= 6 && !perks.inventory {
        perks.inventory = true;
        if let Ok(mut inv) = inventory.single_mut() {
            inv.expand_slots(EXTRA_INVENTORY_SLOTS);
        }
        info!("Mutation perk: +{EXTRA_INVENTORY_SLOTS} inventory slots");
    }
    if floor.number >= 9 && !perks.night_vision {
        perks.night_vision = true;
        info!("Mutation perk: night vision");
    }
}
