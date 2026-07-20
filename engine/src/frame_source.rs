//! What paints the 320×200 framebuffer each frame.

use bevy::prelude::*;

/// Selects the CPU framebuffer path.
///
/// Defaults to [`AttractTitle`] so boot lands on the NES-style title backdrop
/// before a floor is loaded. Gameplay flips this to [`Raycast`] in-game.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FrameSource {
    /// Static/slow-animated title attract (menus, boot).
    #[default]
    AttractTitle,
    /// Full software raycaster (InGame).
    Raycast,
}
