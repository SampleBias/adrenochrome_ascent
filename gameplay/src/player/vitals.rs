//! Health, armor, and inventory (TODO-012).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::weapons::{AmmoType, WeaponId};

pub const MAX_HEALTH: f32 = 100.0;
pub const MAX_ARMOR: f32 = 100.0;
pub const INVENTORY_SLOTS: usize = 8;

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: MAX_HEALTH,
            max: MAX_HEALTH,
        }
    }
}

impl Health {
    pub fn is_dead(self) -> bool {
        self.current <= 0.0
    }

    pub fn ratio(self) -> f32 {
        (self.current / self.max).clamp(0.0, 1.0)
    }
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Armor {
    pub current: f32,
    pub max: f32,
}

impl Default for Armor {
    fn default() -> Self {
        Self {
            current: 0.0,
            max: MAX_ARMOR,
        }
    }
}

/// Apply damage: armor absorbs 2/3, health takes the rest (Doom-ish).
pub fn apply_damage(health: &mut Health, armor: &mut Armor, amount: f32) -> f32 {
    if amount <= 0.0 {
        return 0.0;
    }
    let mut remaining = amount;
    if armor.current > 0.0 {
        let absorbed = (remaining * 0.66).min(armor.current);
        armor.current -= absorbed;
        remaining -= absorbed;
    }
    health.current = (health.current - remaining).max(0.0);
    amount
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemKind {
    Keycard,
    KeyItem(String),
    Weapon(WeaponId),
    Ammo { kind: AmmoType, count: u32 },
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub slots: Vec<Option<ItemKind>>,
    pub ammo_9mm: u32,
    pub ammo_shells: u32,
    pub ammo_cells: u32,
    /// Injector "doses" (uses as ammo).
    pub ammo_adreno: u32,
}

impl Default for Inventory {
    fn default() -> Self {
        let mut slots = vec![None; INVENTORY_SLOTS];
        slots[0] = Some(ItemKind::Weapon(WeaponId::Pistol));
        Self {
            slots,
            ammo_9mm: 12,
            ammo_shells: 0,
            ammo_cells: 0,
            ammo_adreno: 1,
        }
    }
}

impl Inventory {
    pub fn ammo_for(&self, kind: AmmoType) -> u32 {
        match kind {
            AmmoType::Bullet9mm => self.ammo_9mm,
            AmmoType::Shell => self.ammo_shells,
            AmmoType::Cell => self.ammo_cells,
            AmmoType::Adreno => self.ammo_adreno,
        }
    }

    pub fn ammo_for_mut(&mut self, kind: AmmoType) -> &mut u32 {
        match kind {
            AmmoType::Bullet9mm => &mut self.ammo_9mm,
            AmmoType::Shell => &mut self.ammo_shells,
            AmmoType::Cell => &mut self.ammo_cells,
            AmmoType::Adreno => &mut self.ammo_adreno,
        }
    }

    pub fn try_consume(&mut self, kind: AmmoType, amount: u32) -> bool {
        let pool = self.ammo_for_mut(kind);
        if *pool >= amount {
            *pool -= amount;
            true
        } else {
            false
        }
    }

    pub fn has_weapon(&self, id: WeaponId) -> bool {
        self.slots.iter().any(|s| matches!(s, Some(ItemKind::Weapon(w)) if *w == id))
    }

    pub fn grant_weapon(&mut self, id: WeaponId) {
        if self.has_weapon(id) {
            return;
        }
        if let Some(slot) = self.slots.iter_mut().find(|s| s.is_none()) {
            *slot = Some(ItemKind::Weapon(id));
        }
    }

    /// Permanent inventory expansion (mutation perk, TODO-030).
    pub fn expand_slots(&mut self, extra: usize) {
        self.slots.extend(std::iter::repeat(None).take(extra));
    }
}

/// Red pain flash intensity 0..1 (TODO-012).
#[derive(Resource, Debug, Clone, Copy, Default)]
pub struct PainFlash {
    pub intensity: f32,
}

impl PainFlash {
    pub fn trigger(&mut self, strength: f32) {
        self.intensity = self.intensity.max(strength.clamp(0.0, 1.0));
    }
}

pub fn tick_pain_flash(time: Res<Time>, mut pain: ResMut<PainFlash>) {
    if pain.intensity > 0.0 {
        pain.intensity = (pain.intensity - time.delta_secs() * 1.8).max(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armor_absorbs_majority_of_damage() {
        let mut health = Health::default();
        let mut armor = Armor {
            current: 50.0,
            max: MAX_ARMOR,
        };
        apply_damage(&mut health, &mut armor, 30.0);
        assert!(armor.current < 50.0);
        assert!(health.current > 70.0);
        assert!(health.current < 100.0);
    }

    #[test]
    fn inventory_starts_with_pistol_and_consumes_ammo() {
        let mut inv = Inventory::default();
        assert!(inv.has_weapon(WeaponId::Pistol));
        assert!(inv.try_consume(AmmoType::Bullet9mm, 1));
        assert_eq!(inv.ammo_9mm, 11);
        assert!(!inv.try_consume(AmmoType::Shell, 1));
    }
}
