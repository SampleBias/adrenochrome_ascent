//! Top-level game flow states (TODO-005).
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
    /// Between-floor ride (load/save + palette/audio shift; save is TODO-010).
    ElevatorTransition,
    /// Post-run ending screen (branches via [`EndingKind`](super::floor::EndingKind)).
    Ending,
}
