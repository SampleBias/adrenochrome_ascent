//! Player controller, vitals, weapons, and viewmodel (TODO-004 / Sprint 3).

mod constants;
mod controller;
mod hand;
mod perks;
mod serum;
mod vitals;
mod weapons;

pub use constants::*;
pub use controller::{
    Player, PlayerMotor, PlayerSet, capture_mouse, player_look, player_move, release_mouse,
    spawn_player, toggle_cursor_grab,
};
pub use hand::{sync_pain_flash_ui, update_hand_viewmodel, HandState, PainFlashUi};
pub use perks::{grant_mutation_perks, MutationPerks};
pub use serum::{
    apply_serum, cure_serum, sync_serum_overlay_ui, tick_serum_effect, SerumEffect, SerumOverlayUi,
};
pub use vitals::{
    apply_damage, tick_pain_flash, Armor, Health, Inventory, ItemKind, PainFlash, INVENTORY_SLOTS,
    MAX_ARMOR, MAX_HEALTH,
};
pub use weapons::{
    debug_grant_weapons, fire_weapon, select_weapon, tick_adreno_vision, tick_weapon_timers,
    weapon_stats, AdrenoVision, AmmoType, MuzzleFlash, ScreenShake, WeaponId, WeaponLoadout,
    WeaponStats,
};
