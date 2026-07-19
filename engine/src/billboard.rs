//! Billboard sprites for the software raycaster.
//!
//! World billboards (enemies, items, wisps) are projected with the classic
//! Doom sprite transform and depth-tested against the wall z-buffer.
//! The player hand is a screen-space overlay drawn after world sprites.

use bevy::prelude::*;

/// World-space billboard rendered by the raycaster.
#[derive(Component, Debug, Clone, Copy)]
pub struct Billboard {
    /// Position in map-cell units (same space as [`crate::ray_camera::RayCamera`]).
    pub pos: Vec2,
    /// Index into [`crate::textures::TextureSet::sprites`].
    pub texture_id: usize,
    /// World scale (1.0 ≈ one cell tall).
    pub scale: f32,
    /// Hit-reaction flash intensity 0..1 (white/red tint).
    pub flash: f32,
}

impl Billboard {
    pub fn new(pos: Vec2, texture_id: usize, scale: f32) -> Self {
        Self {
            pos,
            texture_id,
            scale,
            flash: 0.0,
        }
    }
}

/// Camera-fixed hand / weapon viewmodel overlay.
#[derive(Component, Debug, Clone, Copy)]
pub struct HandOverlay {
    pub texture_id: usize,
    /// Anchor on screen in 0..1 UV (bottom-right default).
    pub anchor: Vec2,
    pub scale: f32,
    /// Magenta glow pulse speed (radians/sec). 0 = static.
    pub glow_pulse: f32,
    /// Muzzle flash intensity 0..1 drawn over the viewmodel (TODO-014).
    pub muzzle: f32,
}

impl Default for HandOverlay {
    fn default() -> Self {
        Self {
            texture_id: 1,
            anchor: Vec2::new(0.78, 0.92),
            scale: 1.15,
            glow_pulse: 2.5,
            muzzle: 0.0,
        }
    }
}
