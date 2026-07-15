use bevy::prelude::*;

/// Marker for an interactable puzzle object in the world.
///
/// Each level has at least one. When the player presses `E` within
/// `INTERACT_DISTANCE`, the puzzle's `solve` logic is triggered.
#[derive(Component, Debug, Clone)]
pub struct PuzzleInteractable {
    /// Which level this puzzle belongs to (1..=7).
    pub level: u8,
    /// Whether the puzzle has been solved.
    pub solved: bool,
    /// The prompt text shown in the HUD when the player is near.
    pub prompt: String,
}

/// A keypad / combination lock puzzle (levels 1, 3).
#[derive(Component, Debug, Clone)]
pub struct KeypadPuzzle {
    pub correct_code: String,
    pub entered_code: String,
    pub solved: bool,
}

/// A lever / timing puzzle (level 6 — incinerator).
#[derive(Component, Debug, Clone)]
pub struct TimingPuzzle {
    pub window_start: f32,
    pub window_end: f32,
    pub elapsed: f32,
    pub solved: bool,
}

/// A circuit / hacking grid puzzle (level 5 — server room).
#[derive(Component, Debug, Clone)]
pub struct CircuitPuzzle {
    pub grid_size: u8,
    pub solved: bool,
}

/// Event fired when a puzzle is solved.
#[derive(Event, Debug, Clone)]
pub struct PuzzleSolved {
    pub level: u8,
}

/// Event fired when the player attempts an interaction.
#[derive(Event, Debug, Clone)]
pub struct InteractAttempt;
