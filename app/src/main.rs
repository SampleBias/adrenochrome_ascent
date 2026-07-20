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

use std::path::PathBuf;

use bevy::{prelude::*, window::PresentMode};

use adrenochrome_content::ContentPlugin;
use adrenochrome_engine::EnginePlugin;
use adrenochrome_gameplay::GameplayPlugin;

/// Point Bevy at the workspace `assets/` folder for both `cargo run` and
/// running the binary from `target/debug`.
fn configure_asset_root() {
    if std::env::var_os("BEVY_ASSET_ROOT").is_some() {
        return;
    }
    let candidates = [
        // `cargo run` — CARGO_MANIFEST_DIR is `app/`.
        std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).map(|p| p.join("..")),
        // Cwd is workspace root.
        Some(PathBuf::from(".")),
        // Binary launched from `target/debug`.
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("../.."))),
    ];
    for root in candidates.into_iter().flatten() {
        if root.join("assets").is_dir() {
            std::env::set_var("BEVY_ASSET_ROOT", &root);
            return;
        }
    }
}

fn main() {
    configure_asset_root();

    App::new()
        // Bevy core plugins: rendering, input, windowing, assets, audio, UI.
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Adrenochrome Ascent".to_string(),
                        resolution: (1280u32, 720u32).into(),
                        resizable: true,
                        // Mailbox avoids swapchain acquire timeouts common on
                        // NVIDIA + Wayland when AutoVsync backs up under load.
                        present_mode: PresentMode::Mailbox,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "assets".into(),
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
