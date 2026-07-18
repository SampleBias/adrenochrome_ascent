//! Lightweight state-overlay UI (TODO-005). Full pixel HUD is TODO-033.

use bevy::prelude::*;

use crate::game::{CurrentFloor, EndingKind, GameState};

/// Marker for state overlay UI roots (despawned on state exit).
#[derive(Component, Debug, Clone, Copy)]
pub struct MenuUi;

fn title_font(asset_server: &AssetServer, size: f32) -> TextFont {
    TextFont {
        font: FontSource::Handle(asset_server.load("fonts/FiraSans-Bold.ttf")),
        font_size: FontSize::Px(size),
        ..default()
    }
}

fn fullscreen_root(bg: Color) -> (Node, BackgroundColor) {
    (
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(12),
            ..default()
        },
        BackgroundColor(bg),
    )
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title = title_font(&asset_server, 42.0);
    let body = title_font(&asset_server, 18.0);

    commands
        .spawn((
            Name::new("MainMenuUi"),
            MenuUi,
            DespawnOnExit(GameState::MainMenu),
            fullscreen_root(Color::srgba(0.02, 0.01, 0.02, 0.82)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("ADRENOCHROME ASCENT"),
                title,
                TextColor(Color::srgb(0.75, 0.12, 0.14)),
            ));
            parent.spawn((
                Text::new(
                    "Wake in the basement. Solve. Survive. Ascend.\n\n[ENTER] New Game    [ESC] Quit",
                ),
                body,
                TextColor(Color::srgb(0.72, 0.68, 0.70)),
            ));
        });
}

pub fn spawn_elevator_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    floor: Res<CurrentFloor>,
) {
    let title = title_font(&asset_server, 28.0);
    let body = title_font(&asset_server, 16.0);
    let destination = if floor.number >= CurrentFloor::MAX {
        "SURFACE — ENDING".to_string()
    } else {
        format!("Floor {}", floor.number + 1)
    };

    commands
        .spawn((
            Name::new("ElevatorUi"),
            MenuUi,
            DespawnOnExit(GameState::ElevatorTransition),
            fullscreen_root(Color::srgba(0.0, 0.0, 0.0, 0.55)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("ELEVATOR"),
                title,
                TextColor(Color::srgb(0.55, 0.85, 0.55)),
            ));
            parent.spawn((
                Text::new(format!(
                    "Leaving floor {} ({})\nNext: {}\n\n[ENTER] Skip",
                    floor.number,
                    floor.cluster_name(),
                    destination
                )),
                body,
                TextColor(Color::srgb(0.8, 0.8, 0.75)),
            ));
        });
}

pub fn spawn_ending(mut commands: Commands, asset_server: Res<AssetServer>, ending: Res<EndingKind>) {
    let title = title_font(&asset_server, 34.0);
    let body = title_font(&asset_server, 18.0);

    commands
        .spawn((
            Name::new("EndingUi"),
            MenuUi,
            DespawnOnExit(GameState::Ending),
            fullscreen_root(Color::srgba(0.01, 0.02, 0.03, 0.88)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(ending.title()),
                title,
                TextColor(Color::srgb(0.85, 0.75, 0.55)),
            ));
            parent.spawn((
                Text::new(format!(
                    "{}\n\n[ENTER] Main Menu",
                    ending.blurb()
                )),
                body,
                TextColor(Color::srgb(0.7, 0.7, 0.72)),
            ));
        });
}

/// Tiny in-game floor readout (top-left).
pub fn spawn_ingame_hud(mut commands: Commands, asset_server: Res<AssetServer>, floor: Res<CurrentFloor>) {
    let font = title_font(&asset_server, 14.0);
    commands
        .spawn((
            Name::new("InGameHud"),
            MenuUi,
            DespawnOnExit(GameState::InGame),
            Node {
                position_type: PositionType::Absolute,
                top: px(10),
                left: px(12),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!(
                    "FLOOR {} — {}\n[L] Elevator   [TAB] Cursor",
                    floor.number,
                    floor.cluster_name()
                )),
                font,
                TextColor(Color::srgba(0.85, 0.8, 0.75, 0.85)),
            ));
        });
}
