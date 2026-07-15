use bevy::prelude::*;

use crate::game::states::GameState;

/// Static definition of a level.
#[derive(Debug, Clone)]
pub struct LevelDefinition {
    pub number: u8,
    pub name: &'static str,
    pub subtitle: &'static str,
    pub state: GameState,
    /// Where the player spawns.
    pub player_spawn: Vec3,
    /// Ambient light color.
    pub ambient_light: [f32; 3],
    /// Ambient light brightness.
    pub ambient_brightness: f32,
    /// A short narrative blurb shown as a subtitle on entry.
    pub intro_text: &'static str,
}

impl LevelDefinition {
    /// Returns the definition for the given game state, or `None`.
    pub fn for_state(state: GameState) -> Option<Self> {
        match state {
            GameState::Level1 => Some(Self {
                number: 1,
                name: "Awakening",
                subtitle: "Basement — Medical Bay",
                state,
                player_spawn: Vec3::new(0.0, 1.7, 0.0),
                ambient_light: [0.3, 0.3, 0.35],
                ambient_brightness: 0.15,
                intro_text: "You wake up chained to a bed. The restraints are loose...",
            }),
            GameState::Level2 => Some(Self {
                number: 2,
                name: "The Corridor",
                subtitle: "Basement — Hallway B",
                state,
                player_spawn: Vec3::new(0.0, 1.7, -8.0),
                ambient_light: [0.25, 0.25, 0.3],
                ambient_brightness: 0.12,
                intro_text: "A scientist patrols the corridor. Stay in the shadows.",
            }),
            GameState::Level3 => Some(Self {
                number: 3,
                name: "Storage Room",
                subtitle: "Basement — Storage",
                state,
                player_spawn: Vec3::new(-5.0, 1.7, -5.0),
                ambient_light: [0.2, 0.2, 0.25],
                ambient_brightness: 0.10,
                intro_text: "Keycards are scattered here. A lock demands a code.",
            }),
            GameState::Level4 => Some(Self {
                number: 4,
                name: "Laboratory",
                subtitle: "Basement — Lab 3",
                state,
                player_spawn: Vec3::new(0.0, 1.7, 5.0),
                ambient_light: [0.3, 0.35, 0.4],
                ambient_brightness: 0.18,
                intro_text: "Chemicals line the shelves. Cameras watch every shelf.",
            }),
            GameState::Level5 => Some(Self {
                number: 5,
                name: "Server Room",
                subtitle: "Basement — Data Center",
                state,
                player_spawn: Vec3::new(6.0, 1.7, 6.0),
                ambient_light: [0.15, 0.2, 0.3],
                ambient_brightness: 0.10,
                intro_text: "Terminals hum. The circuit lock guards the exit.",
            }),
            GameState::Level6 => Some(Self {
                number: 6,
                name: "Incinerator",
                subtitle: "Basement — Disposal",
                state,
                player_spawn: Vec3::new(-6.0, 1.7, 6.0),
                ambient_light: [0.4, 0.2, 0.15],
                ambient_brightness: 0.14,
                intro_text: "The incinerator roars. Time the burn. Destroy the evidence.",
            }),
            GameState::Level7 => Some(Self {
                number: 7,
                name: "The Surface",
                subtitle: "Ground Floor — Exit",
                state,
                player_spawn: Vec3::new(0.0, 1.7, 0.0),
                ambient_light: [0.5, 0.45, 0.4],
                ambient_brightness: 0.25,
                intro_text: "Alarms scream. The surface door is close. Don't stop.",
            }),
            _ => None,
        }
    }

    /// All 7 levels in order.
    pub fn all() -> [Self; 7] {
        [
            Self::for_state(GameState::Level1).unwrap(),
            Self::for_state(GameState::Level2).unwrap(),
            Self::for_state(GameState::Level3).unwrap(),
            Self::for_state(GameState::Level4).unwrap(),
            Self::for_state(GameState::Level5).unwrap(),
            Self::for_state(GameState::Level6).unwrap(),
            Self::for_state(GameState::Level7).unwrap(),
        ]
    }
}
