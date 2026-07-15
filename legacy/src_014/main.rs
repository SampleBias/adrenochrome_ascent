//! Adrenochrome Ascent — a first-person escape-room horror puzzle game.
//!
//! You wake up chained to a bed in the basement of an experimental laboratory.
//! Solve puzzles across 7 levels, avoid detection, and ascend to the surface.
//!
//! Built with Bevy 0.14.

// This is a framework scaffold: many structs/constants are intentionally
// unused yet — they form the API surface for per-level content authoring.
#![allow(dead_code)]

use bevy::prelude::*;

mod audio;
mod enemy;
mod game;
mod level;
mod player;
mod puzzle;
mod ui;

use audio::plugin::AudioPlugin;
use enemy::plugin::EnemyPlugin;
use game::plugin::CoreGamePlugin;
use game::states::GameState;
use level::plugin::LevelPlugin;
use player::plugin::PlayerPlugin;
use puzzle::plugin::PuzzlePlugin;
use ui::plugin::UiPlugin;

fn main() {
    App::new()
        // Bevy core plugins. We use DefaultPlugins so we get rendering,
        // input, windowing, asset loading, and UI out of the box.
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Adrenochrome Ascent".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // Game flow state machine.
        .init_state::<GameState>()
        // Our plugins.
        .add_plugins((
            CoreGamePlugin,
            PlayerPlugin,
            LevelPlugin,
            PuzzlePlugin,
            EnemyPlugin,
            UiPlugin,
            AudioPlugin,
        ))
        .run();
}
