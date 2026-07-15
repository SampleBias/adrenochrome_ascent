use bevy::prelude::*;

use crate::game::states::GameState;

/// Marker for menu UI entities.
#[derive(Component, Debug, Clone, Copy)]
pub struct MenuEntity;

/// Spawns the main menu.
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Name::new("MainMenu"),
            MenuEntity,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.02, 0.02, 0.03).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title.
            parent.spawn(TextBundle::from_section(
                "ADRENOCHROME ASCENT",
                TextStyle {
                    font: font.clone(),
                    font_size: 48.0,
                    color: Color::srgb(0.7, 0.1, 0.1),
                },
            ));

            parent.spawn(TextBundle::from_section(
                "\nYou wake up chained to a bed in the basement of an experimental laboratory.\n\nSolve puzzles. Avoid detection. Ascend.\n\n[Press ENTER to begin]  [Press ESC to quit]",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                },
            ));
        });
}

/// Spawns the pause menu.
pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Name::new("PauseMenu"),
            MenuEntity,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "PAUSED\n\n[Press ESC to resume]  [Press Q to quit to menu]",
                TextStyle {
                    font,
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            ));
        });
}

/// Spawns the game over screen.
pub fn spawn_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Name::new("GameOver"),
            MenuEntity,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.1, 0.0, 0.0).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "YOU WERE CAUGHT\n\n[Press ENTER to retry]  [Press Q for main menu]",
                TextStyle {
                    font,
                    font_size: 36.0,
                    color: Color::srgb(0.9, 0.2, 0.2),
                },
            ));
        });
}

/// Spawns the victory screen.
pub fn spawn_victory(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            Name::new("Victory"),
            MenuEntity,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb(0.0, 0.05, 0.02).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "YOU ESCAPED\n\nAdrenochrome Ascent — Complete\n\n[Press ENTER for main menu]",
                TextStyle {
                    font,
                    font_size: 36.0,
                    color: Color::srgb(0.2, 0.9, 0.3),
                },
            ));
        });
}

/// Despawns all menu entities.
pub fn despawn_menus(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Handles menu input (Enter to start/retry, Esc to pause/resume, Q to quit).
pub fn menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    let current = state.get();

    match current {
        GameState::MainMenu => {
            if keys.just_pressed(KeyCode::Enter) {
                next_state.set(GameState::Level1);
            }
            if keys.just_pressed(KeyCode::Escape) {
                exit.send(AppExit::Success);
            }
        }
        GameState::Paused => {
            if keys.just_pressed(KeyCode::Escape) {
                // Resume — return to the level we were on.
                // TODO: store the paused level; for now go to Level1.
                next_state.set(GameState::Level1);
            }
            if keys.just_pressed(KeyCode::KeyQ) {
                next_state.set(GameState::MainMenu);
            }
        }
        GameState::GameOver => {
            if keys.just_pressed(KeyCode::Enter) {
                next_state.set(GameState::Level1);
            }
            if keys.just_pressed(KeyCode::KeyQ) {
                next_state.set(GameState::MainMenu);
            }
        }
        GameState::Victory => {
            if keys.just_pressed(KeyCode::Enter) {
                next_state.set(GameState::MainMenu);
            }
        }
        _ => {
            // In-game: Esc pauses.
            if keys.just_pressed(KeyCode::Escape) {
                next_state.set(GameState::Paused);
            }
        }
    }
}
