//! Floor palettes for the CRT render pipeline.
//!
//! Each floor cluster shifts the overall color grade of the scene. Tuned to
//! the lo-fi horror references in `assets/images/style_reference/`:
//! blood-red halls, toxic green lobbies, sterile teal liminal corridors,
//! and cold void black.
//!
//! The CRT upscale shader reads the active palette from a uniform and remaps
//! the low-res render target's colors.

use bevy::prelude::*;

/// Internal low-res render resolution. 320×200 matches the retro CRT target.
pub const RENDER_WIDTH: u32 = 320;
pub const RENDER_HEIGHT: u32 = 200;

/// Which floor cluster's palette is currently active.
///
/// Progression: red → green → teal → black (surface / void).
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
    /// Floors 1-3 (Human tier): arterial blood red / maroon halls.
    #[default]
    Red,
    /// Floors 4-7 (Hybrid tier): sickly chemical green.
    Green,
    /// Floors 8-9 (Surface approach): sterile teal / liminal gloom.
    Teal,
    /// Floor 10 (Surface): cold black / void.
    Black,
}

impl Palette {
    /// Linear RGBA tint applied to the whole frame by the CRT shader.
    ///
    /// Stronger than a soft grade — these push the scene toward the reference
    /// screenshots while still letting authored sprite colors read through.
    pub fn tint(self) -> [f32; 4] {
        match self {
            // Blood and shadow (refs 02, 03).
            Palette::Red => [1.15, 0.38, 0.36, 1.0],
            // Radioactive lobby green (ref 01).
            Palette::Green => [0.42, 1.05, 0.48, 1.0],
            // Cold checkered-hall teal (ref 04).
            Palette::Teal => [0.40, 0.78, 0.95, 1.0],
            // Void / nearly monochrome with a blue death cast (ref 05 shadow).
            Palette::Black => [0.62, 0.64, 0.78, 1.0],
        }
    }

    /// Advance to the next palette in the progression.
    pub fn next(self) -> Self {
        match self {
            Palette::Red => Palette::Green,
            Palette::Green => Palette::Teal,
            Palette::Teal => Palette::Black,
            Palette::Black => Palette::Red,
        }
    }
}
