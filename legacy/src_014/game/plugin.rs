use bevy::prelude::*;

use crate::game::states::GameState;

/// Core game plugin: wires up shared resources and cross-cutting systems.
pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, app: &mut App) {
        // Global detection meter resource.
        app.insert_resource(DetectionMeter::default())
            .insert_resource(CurrentLevelInfo::default())
            // Reset detection when entering any playable level.
            .add_systems(OnEnter(GameState::Level1), reset_run_state)
            .add_systems(OnEnter(GameState::Level2), reset_run_state)
            .add_systems(OnEnter(GameState::Level3), reset_run_state)
            .add_systems(OnEnter(GameState::Level4), reset_run_state)
            .add_systems(OnEnter(GameState::Level5), reset_run_state)
            .add_systems(OnEnter(GameState::Level6), reset_run_state)
            .add_systems(OnEnter(GameState::Level7), reset_run_state);
    }
}

/// Global detection meter. When it reaches `DETECTION_MAX` the player is
/// caught and the game transitions to `GameOver`.
#[derive(Resource, Debug, Clone, Copy)]
pub struct DetectionMeter {
    pub value: f32,
}

impl Default for DetectionMeter {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl DetectionMeter {
    pub fn ratio(&self) -> f32 {
        use crate::game::constants::DETECTION_MAX;
        (self.value / DETECTION_MAX).clamp(0.0, 1.0)
    }

    pub fn is_caught(&self) -> bool {
        use crate::game::constants::DETECTION_MAX;
        self.value >= DETECTION_MAX
    }
}

/// Metadata about the currently loaded level.
#[derive(Resource, Debug, Clone, Default)]
pub struct CurrentLevelInfo {
    pub number: u8,
    pub name: String,
    pub subtitle: String,
}

fn reset_run_state(mut detection: ResMut<DetectionMeter>) {
    detection.value = 0.0;
}
