//! Floor palettes for the CRT render pipeline.
//!
//! Each floor cluster (per TODO-006) shifts the overall color grade of the
//! scene. The CRT upscale shader (see `crt_material`) reads the active
//! palette from a uniform and remaps the low-res render target's colors.
//!
//! TODO-002 only wires up the palette *data* and a demo swap system; the
//! actual per-floor binding arrives with the floor loader in TODO-007.

use bevy::prelude::*;

/// Internal low-res render resolution. 320×200 matches the retro CRT target
/// called out in TODO-002. Exposed publicly so gameplay/systems that reason
/// about the render target (e.g. the raycaster in TODO-003) share one source
/// of truth.
pub const RENDER_WIDTH: u32 = 320;
pub const RENDER_HEIGHT: u32 = 200;

/// Which floor cluster's palette is currently active.
///
/// The progression red → green → teal → black is described in TODO-006.
/// `Black` is the final "surface" cluster (floors 8-10).
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivePalette {
    pub palette: Palette,
}

impl ActivePalette {
    pub fn new(palette: Palette) -> Self {
        Self { palette }
    }
}

/// The four floor-cluster palettes. Colors are stored as linear RGBA f32x4
/// so they can be uploaded directly to the shader uniform without conversion.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Palette {
    /// Floors 1-3 (Human tier): sickly red.
    #[default]
    Red,
    /// Floors 4-7 (Hybrid tier): chemical green.
    Green,
    /// Floors 8-9 (Surface approach): sterile teal.
    Teal,
    /// Floor 10 (Surface): cold black/white.
    Black,
}

impl Palette {
    /// Linear RGBA tint applied to the whole frame by the CRT shader.
    ///
    /// These are deliberately subtle multipliers — they grade the scene
    /// rather than replace it, so authored sprite colors still read through.
    pub fn tint(self) -> [f32; 4] {
        match self {
            // Deep arterial red.
            Palette::Red => [1.0, 0.55, 0.55, 1.0],
            // Toxic lab green.
            Palette::Green => [0.55, 1.0, 0.55, 1.0],
            // Cold sterile teal.
            Palette::Teal => [0.55, 0.85, 1.0, 1.0],
            // Near-monochrome, slightly blue black.
            Palette::Black => [0.75, 0.75, 0.85, 1.0],
        }
    }

    /// Advance to the next palette in the progression. Used by the demo
    /// swap system in TODO-002 and, later, by elevator transitions.
    pub fn next(self) -> Self {
        match self {
            Palette::Red => Palette::Green,
            Palette::Green => Palette::Teal,
            Palette::Teal => Palette::Black,
            Palette::Black => Palette::Red,
        }
    }
}
