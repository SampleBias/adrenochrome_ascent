//! Gameplay UI overlays for game-state flow, pixel HUD sync, egui terminals.

mod hud;
mod level_map;
mod menus;
mod terminal;

pub use hud::{disable_pixel_hud, sync_crt_post_fx, sync_pixel_hud};
pub use level_map::{
    handle_level_map_input, level_map_not_open, map_is_unlocked, map_unlock_expr,
    reset_level_map_on_exit, LevelMapState,
};
pub use menus::*;
pub use terminal::{TerminalSession, TerminalUiPlugin};
