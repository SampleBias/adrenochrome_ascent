//! Adrenochrome Ascent — entry point.
//!
//! Retro first-person horror-action game built in Rust + Bevy 0.19.
//!
//! Core Loop: Explore 10 floors, solve 3-5 interconnected puzzles per floor,
//! survive limited combat, reach elevator, ascend. Custom software raycaster
//! renders to a 320×200 internal target, upscaled with a CRT shader.
//!
//! Plugins: engine (raycaster/CRT), gameplay (player + GameState flow), content.

#![allow(dead_code)]

use bevy::prelude::*;

use adrenochrome_content::ContentPlugin;
use adrenochrome_engine::EnginePlugin;
use adrenochrome_gameplay::GameplayPlugin;

fn main() {
    App::new()
        // Bevy core plugins: rendering, input, windowing, assets, audio, UI.
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Adrenochrome Ascent".to_string(),
                    resolution: (1280u32, 720u32).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
        )
        // Engine: raycaster + 320×200 render target + CRT shader (TODO-002/003).
        .add_plugins(EnginePlugin)
        // Gameplay: player controller, state machine, enemies, puzzles (TODO-004/005).
        .add_plugins(GameplayPlugin)
        // Content: floor RON data, asset definitions (TODO-006/007).
        .add_plugins(ContentPlugin)
        .run();
}
