//! Autosave on elevator rides — 10 RON slots (TODO-010 / Sprint 3–7).

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use adrenochrome_engine::RayCamera;

use crate::enemy::{Faction, FactionRegistry};
use crate::game::{CurrentFloor, EndingKind};
use crate::player::{
    Armor, Health, Inventory, MutationPerks, Player, PlayerMotor, WeaponId, WeaponLoadout,
};
use crate::puzzle::PuzzleRegistry;

/// Which of the 10 save slots receives autosaves (1..=10).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveSaveSlot {
    pub slot: u8,
}

impl Default for ActiveSaveSlot {
    fn default() -> Self {
        Self { slot: 1 }
    }
}

impl ActiveSaveSlot {
    pub fn path(self) -> PathBuf {
        PathBuf::from("saves").join(format!("slot_{:02}.ron", self.slot.clamp(1, 10)))
    }
}

/// Serializable run state written during elevator transitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGame {
    pub slot: u8,
    pub floor: u8,
    pub player_pos: (f32, f32),
    pub player_yaw: f32,
    pub ending: EndingKind,
    pub puzzle_flags: HashMap<String, bool>,
    #[serde(default)]
    pub puzzle_counters: HashMap<String, i32>,
    pub health: f32,
    pub armor: f32,
    pub inventory: Inventory,
    pub weapon: WeaponId,
    #[serde(default)]
    pub perks: MutationPerks,
    #[serde(default)]
    pub factions_defeated: HashSet<Faction>,
    #[serde(default)]
    pub factions_spared: HashSet<Faction>,
}

/// Write the active slot during elevator enter (after fade begins).
pub fn autosave_on_elevator(
    slot: Res<ActiveSaveSlot>,
    floor: Res<CurrentFloor>,
    camera: Res<RayCamera>,
    registry: Res<PuzzleRegistry>,
    ending: Res<EndingKind>,
    perks: Res<MutationPerks>,
    factions: Res<FactionRegistry>,
    players: Query<
        (&PlayerMotor, &Health, &Armor, &Inventory, &WeaponLoadout),
        With<Player>,
    >,
) {
    let (pos, yaw, health, armor, inventory, weapon) =
        if let Some((m, h, a, inv, loadout)) = players.iter().next() {
            (
                (m.pos.x, m.pos.y),
                m.yaw,
                h.current,
                a.current,
                inv.clone(),
                loadout.current,
            )
        } else {
            (
                (camera.pos.x, camera.pos.y),
                camera.yaw(),
                100.0,
                0.0,
                Inventory::default(),
                WeaponId::Pistol,
            )
        };

    let save = SaveGame {
        slot: slot.slot.clamp(1, 10),
        floor: floor.number,
        player_pos: pos,
        player_yaw: yaw,
        ending: *ending,
        puzzle_flags: registry.flags.clone(),
        puzzle_counters: registry.counters.clone(),
        health,
        armor,
        inventory,
        weapon,
        perks: *perks,
        factions_defeated: factions.defeated.clone(),
        factions_spared: factions.spared.clone(),
    };

    if let Err(e) = write_save(&slot.path(), &save) {
        error!("Autosave failed: {e}");
    } else {
        info!("Autosaved slot {} (floor {})", save.slot, save.floor);
    }
}

pub fn write_save(path: &Path, save: &SaveGame) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let pretty = ron::ser::PrettyConfig::new().depth_limit(4);
    let text = ron::ser::to_string_pretty(save, pretty).map_err(|e| e.to_string())?;
    std::fs::write(path, text).map_err(|e| e.to_string())
}

pub fn read_save(path: &Path) -> Result<SaveGame, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    ron::from_str(&text).map_err(|e| e.to_string())
}

/// Apply a save into live resources (for future Load Game UI).
#[allow(dead_code)]
pub fn apply_save(
    save: &SaveGame,
    floor: &mut CurrentFloor,
    ending: &mut EndingKind,
    registry: &mut PuzzleRegistry,
    perks: &mut MutationPerks,
    factions: &mut FactionRegistry,
    camera: &mut RayCamera,
    players: &mut Query<
        (
            &mut PlayerMotor,
            &mut Health,
            &mut Armor,
            &mut Inventory,
            &mut WeaponLoadout,
        ),
        With<Player>,
    >,
) {
    floor.number = save.floor.clamp(1, CurrentFloor::MAX);
    *ending = save.ending;
    registry.flags = save.puzzle_flags.clone();
    registry.counters = save.puzzle_counters.clone();
    *perks = save.perks;
    factions.defeated = save.factions_defeated.clone();
    factions.spared = save.factions_spared.clone();
    let pos = Vec2::new(save.player_pos.0, save.player_pos.1);
    *camera = RayCamera::from_yaw(pos, save.player_yaw);
    for (mut motor, mut health, mut armor, mut inventory, mut loadout) in players.iter_mut() {
        motor.pos = pos;
        motor.yaw = save.player_yaw;
        motor.velocity = Vec2::ZERO;
        health.current = save.health;
        armor.current = save.armor;
        *inventory = save.inventory.clone();
        loadout.current = save.weapon;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_save_ron() {
        let dir = std::env::temp_dir().join("adrenochrome_save_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("slot_01.ron");
        let save = SaveGame {
            slot: 1,
            floor: 3,
            player_pos: (1.5, 2.5),
            player_yaw: 1.0,
            ending: EndingKind::Contained,
            puzzle_flags: HashMap::from([("has_keycard".into(), true)]),
            puzzle_counters: HashMap::from([("moral_score".into(), 2)]),
            health: 80.0,
            armor: 25.0,
            inventory: Inventory::default(),
            weapon: WeaponId::Pistol,
            perks: MutationPerks {
                speed: true,
                inventory: false,
                night_vision: false,
            },
            factions_defeated: HashSet::new(),
            factions_spared: HashSet::new(),
        };
        write_save(&path, &save).unwrap();
        let loaded = read_save(&path).unwrap();
        assert_eq!(loaded.floor, 3);
        assert!(loaded.puzzle_flags["has_keycard"]);
        assert_eq!(loaded.puzzle_counters["moral_score"], 2);
        assert!(loaded.perks.speed);
        assert_eq!(loaded.health, 80.0);
        assert_eq!(loaded.armor, 25.0);
        let _ = std::fs::remove_file(&path);
    }
}
