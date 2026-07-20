//! Gameplay UI overlays for game-state flow, pixel HUD sync, egui terminals.

mod hud;
mod menus;
mod terminal;

pub use hud::{disable_pixel_hud, sync_crt_post_fx, sync_pixel_hud};
pub use menus::*;
pub use terminal::{TerminalSession, TerminalUiPlugin};
