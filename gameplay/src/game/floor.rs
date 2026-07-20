//! Floor progression and ending branch data (TODO-005 / TODO-029 hooks).

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use adrenochrome_engine::Palette;

/// Active floor number inside [`super::states::GameState::InGame`] (1..=10).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentFloor {
    pub number: u8,
}

impl Default for CurrentFloor {
    fn default() -> Self {
        Self { number: 1 }
    }
}

impl CurrentFloor {
    pub const MAX: u8 = 10;

    pub fn reset(&mut self) {
        self.number = 1;
    }

    /// Advance one floor. Returns `true` if still in the 1..=10 range.
    pub fn advance(&mut self) -> bool {
        if self.number >= Self::MAX {
            return false;
        }
        self.number += 1;
        true
    }

    /// Palette cluster for this floor (TODO-006 mapping).
    pub fn palette(self) -> Palette {
        match self.number {
            1..=3 => Palette::Red,
            4..=7 => Palette::Green,
            8..=9 => Palette::Teal,
            _ => Palette::Black,
        }
    }

    pub fn cluster_name(self) -> &'static str {
        match self.number {
            1..=3 => "Human",
            4..=7 => "Hybrid",
            8..=9 => "Surface Approach",
            _ => "Surface",
        }
    }
}

/// Moral-choice ending branch (populated by side puzzles in TODO-029/031).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EndingKind {
    /// Default / darker ending until side puzzles flip the flag.
    #[default]
    Contained,
    /// Hopeful ending when `subjects_released` is earned.
    Released,
}

impl EndingKind {
    pub fn title(self) -> &'static str {
        match self {
            EndingKind::Contained => "THE ASCENT ENDS HERE",
            EndingKind::Released => "SUBJECTS RELEASED",
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            EndingKind::Contained => {
                "The convoy swallows the mountain road.\nWhatever you left below stays below."
            }
            EndingKind::Released => {
                "Snow, headlights, open gates.\nSomeone else gets a chance at daylight."
            }
        }
    }

    /// Resolve ending from moral flags / score (TODO-029 / TODO-031).
    pub fn from_registry(registry: &crate::puzzle::PuzzleRegistry) -> Self {
        if registry.get("subjects_released") || registry.counter("moral_score") >= 3 {
            Self::Released
        } else {
            Self::Contained
        }
    }
}

/// Sync [`EndingKind`] from puzzle flags when entering the Ending state.
pub fn resolve_ending_from_flags(
    registry: Res<crate::puzzle::PuzzleRegistry>,
    mut ending: ResMut<EndingKind>,
) {
    *ending = EndingKind::from_registry(&registry);
    info!("Ending resolved: {:?}", *ending);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_advance_and_palette_clusters() {
        let mut floor = CurrentFloor::default();
        assert_eq!(floor.number, 1);
        assert_eq!(floor.palette(), Palette::Red);
        for _ in 0..3 {
            assert!(floor.advance());
        }
        assert_eq!(floor.number, 4);
        assert_eq!(floor.palette(), Palette::Green);
        floor.number = 10;
        assert!(!floor.advance());
        assert_eq!(floor.palette(), Palette::Black);
    }
}
