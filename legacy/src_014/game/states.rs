use bevy::prelude::*;

/// Top-level game flow states.
///
/// `OnEnter` / `OnExit` schedules drive level loading, UI swaps, and player
/// spawning. Each `LevelN` state corresponds to one of the 7 escape-room
/// stages described in the README.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    /// Level 1 — Awakening: break free from restraints.
    Level1,
    /// Level 2 — The Corridor: sneak past a patrolling scientist.
    Level2,
    /// Level 3 — Storage Room: keycards & combination lock.
    Level3,
    /// Level 4 — Laboratory: chemical mixing, evade cameras.
    Level4,
    /// Level 5 — Server Room: hack terminals via circuit puzzle.
    Level5,
    /// Level 6 — Incinerator: timing puzzle, destroy evidence.
    Level6,
    /// Level 7 — The Surface: final escape under full alarm.
    Level7,
    Paused,
    GameOver,
    Victory,
}

impl GameState {
    /// Returns the level number (1..=7) if this is a playable level state.
    pub fn level_number(self) -> Option<u8> {
        match self {
            GameState::Level1 => Some(1),
            GameState::Level2 => Some(2),
            GameState::Level3 => Some(3),
            GameState::Level4 => Some(4),
            GameState::Level5 => Some(5),
            GameState::Level6 => Some(6),
            GameState::Level7 => Some(7),
            _ => None,
        }
    }

    /// Advances to the next level, or `Victory` after level 7.
    pub fn next_level(self) -> GameState {
        match self {
            GameState::Level1 => GameState::Level2,
            GameState::Level2 => GameState::Level3,
            GameState::Level3 => GameState::Level4,
            GameState::Level4 => GameState::Level5,
            GameState::Level5 => GameState::Level6,
            GameState::Level6 => GameState::Level7,
            GameState::Level7 => GameState::Victory,
            other => other,
        }
    }
}
