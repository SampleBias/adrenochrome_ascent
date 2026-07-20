//! Top-level game flow states (TODO-005 / TODO-040).
//!
//! Floor progression lives in [`crate::game::floor::CurrentFloor`], not as
//! separate states. Elevator rides use `ElevatorTransition`; moral endings
//! branch inside `Ending` via [`crate::game::floor::EndingKind`].

use bevy::prelude::*;

/// High-level game flow.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// Title / start screen.
    #[default]
    MainMenu,
    /// Exploring and solving the current floor.
    InGame,
    /// Between-floor ride (autosave + palette/audio shift).
    ElevatorTransition,
    /// Player died — reload or quit.
    GameOver,
    /// Options overlay (volume / CRT).
    Options,
    /// Credits scroll.
    Credits,
    /// Post-run ending (cinematic + text via [`EndingKind`](super::floor::EndingKind)).
    Ending,
}
