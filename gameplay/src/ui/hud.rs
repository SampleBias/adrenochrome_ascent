//! Pixel HUD sync + CRT post-FX from vitals (TODO-033 / TODO-034).

use bevy::prelude::*;

use adrenochrome_engine::{set_crt_post_fx, CrtMaterial, PixelHud};

use crate::audio::PaAnnouncement;
use crate::enemy::{BossFight, ScientistFight, WardenOverrides};
use crate::floor_loader::LoadedFloorInfo;
use crate::game::CurrentFloor;
use crate::interact::InteractionPrompt;
use crate::player::{
    weapon_stats, Armor, Health, Inventory, PainFlash, Player, SerumEffect, WeaponLoadout,
    MAX_ARMOR, MAX_HEALTH,
};
use crate::puzzle::PuzzleRegistry;
use crate::ui::LevelMapState;

/// Push vitals / prompts into the 320×200 [`PixelHud`] buffer.
pub fn sync_pixel_hud(
    floor: Res<CurrentFloor>,
    info: Res<LoadedFloorInfo>,
    prompt: Res<InteractionPrompt>,
    map_state: Res<LevelMapState>,
    pa: Res<PaAnnouncement>,
    fight: Res<BossFight>,
    warden: Res<WardenOverrides>,
    scientist: Res<ScientistFight>,
    registry: Res<PuzzleRegistry>,
    player: Query<(&Health, &Armor, &Inventory, &WeaponLoadout), With<Player>>,
    mut hud: ResMut<PixelHud>,
) {
    hud.enabled = true;
    hud.floor = floor.number;
    hud.floor_label = if info.name.is_empty() {
        floor.cluster_name().to_string()
    } else {
        info.name.clone()
    };

    if let Ok((health, armor, inv, loadout)) = player.single() {
        hud.hp = health.current;
        hud.hp_max = MAX_HEALTH;
        hud.armor = armor.current;
        hud.armor_max = MAX_ARMOR;
        let stats = weapon_stats(loadout.current);
        hud.weapon = stats.name.to_uppercase();
        if hud.weapon.len() > 8 {
            hud.weapon.truncate(8);
        }
        hud.ammo = inv.ammo_for(stats.ammo);
    }

    hud.prompt = if map_state.locked_flash > 0.0 {
        "MAP LOCKED — first challenge".to_string()
    } else {
        prompt.text.clone().unwrap_or_default()
    };
    if hud.prompt.len() > 36 {
        hud.prompt.truncate(36);
    }

    hud.pa_line = if pa.time_left > 0.0 {
        pa.text.clone()
    } else {
        String::new()
    };
    if hud.pa_line.len() > 40 {
        hud.pa_line.truncate(40);
    }

    hud.status = boss_status_line(&fight, &warden, &scientist, &registry);
}

fn boss_status_line(
    fight: &BossFight,
    warden: &WardenOverrides,
    scientist: &ScientistFight,
    registry: &PuzzleRegistry,
) -> String {
    if scientist.active {
        return format!("SCI P{} DNA", scientist.phase);
    }
    if scientist.defeated {
        return "SCI DOWN".into();
    }
    if warden.active {
        return format!("WARDEN P{}", warden.phase);
    }
    if warden.defeated {
        return "WARDEN DOWN".into();
    }
    if fight.active {
        return format!("LT P{}", fight.phase);
    }
    if fight.defeated {
        return "LT DOWN".into();
    }
    let limbs = registry.counter("collected_limb");
    if limbs > 0 {
        return format!("LIMBS {limbs}");
    }
    String::new()
}

/// Drive CRT pain / serum uniforms (TODO-034); replaces fullscreen UI flashes.
pub fn sync_crt_post_fx(
    pain: Res<PainFlash>,
    serum: Res<SerumEffect>,
    mut materials: ResMut<Assets<CrtMaterial>>,
) {
    let serum_amt = if serum.active {
        serum.vision_dark.clamp(0.2, 0.9)
    } else {
        serum.vision_dark * 0.5
    };
    set_crt_post_fx(&mut materials, pain.intensity, serum_amt);
}

/// Clear pixel HUD when leaving gameplay.
pub fn disable_pixel_hud(mut hud: ResMut<PixelHud>) {
    hud.enabled = false;
    hud.pa_line.clear();
    hud.prompt.clear();
    hud.status.clear();
}
