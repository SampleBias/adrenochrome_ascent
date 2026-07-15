use bevy::prelude::*;

use crate::game::plugin::{CurrentLevelInfo, DetectionMeter};
use crate::puzzle::systems::InteractionPrompt;

/// Marker for HUD entities so we can despawn them.
#[derive(Component, Debug, Clone, Copy)]
pub struct HudEntity;

/// Spawns the HUD (crosshair, detection bar, level name, subtitle, prompt).
/// Called on entering any level.
pub fn spawn_hud(world: &mut World) {
    let font = world
        .resource::<AssetServer>()
        .load("fonts/FiraSans-Bold.ttf");

    // Crosshair.
    world.spawn((
        Name::new("Crosshair"),
        HudEntity,
        TextBundle::from_section(
            "+",
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: Color::srgba(0.8, 0.8, 0.8, 0.6),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(50.0),
            left: Val::Percent(50.0),
            ..default()
        }),
    ));

    // Level name (top-left).
    world.spawn((
        Name::new("LevelName"),
        HudEntity,
        TextBundle::from_section(
            "",
            TextStyle {
                font: font.clone(),
                font_size: 22.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    ));

    // Subtitle / intro text (bottom-center).
    world.spawn((
        Name::new("Subtitle"),
        HudEntity,
        TextBundle::from_section(
            "",
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: Color::srgba(0.9, 0.9, 0.9, 0.8),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(60.0),
            left: Val::Percent(35.0),
            right: Val::Percent(35.0),
            ..default()
        }),
    ));

    // Detection meter (bottom-left).
    world.spawn((
        Name::new("DetectionMeter"),
        HudEntity,
        TextBundle::from_section(
            "DETECTION: [          ] 0%",
            TextStyle {
                font: font.clone(),
                font_size: 16.0,
                color: Color::srgb(0.9, 0.3, 0.3),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    ));

    // Interaction prompt (center, below crosshair).
    world.spawn((
        Name::new("InteractionPrompt"),
        HudEntity,
        TextBundle::from_section(
            "",
            TextStyle {
                font,
                font_size: 18.0,
                color: Color::srgb(0.9, 0.85, 0.6),
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(55.0),
            left: Val::Percent(40.0),
            right: Val::Percent(40.0),
            ..default()
        }),
    ));
}

/// Updates HUD text each frame.
pub fn update_hud(
    detection: Res<DetectionMeter>,
    level_info: Res<CurrentLevelInfo>,
    prompt: Res<InteractionPrompt>,
    mut query: Query<(&Name, &mut Text), With<HudEntity>>,
) {
    for (name, mut text) in query.iter_mut() {
        match name.as_str() {
            "LevelName" => {
                text.sections[0].value =
                    format!("Level {}: {}", level_info.number, level_info.name);
            }
            "Subtitle" => {
                text.sections[0].value = level_info.subtitle.clone();
            }
            "DetectionMeter" => {
                let ratio = detection.ratio();
                let pct = (ratio * 100.0) as u32;
                let filled = (ratio * 10.0).round() as usize;
                let bar: String = "=".repeat(filled) + &" ".repeat(10usize.saturating_sub(filled));
                text.sections[0].value = format!("DETECTION: [{}] {}%", bar, pct);
            }
            "InteractionPrompt" => {
                text.sections[0].value = prompt.text.clone().unwrap_or_default();
            }
            _ => {}
        }
    }
}

/// Despawns all HUD entities.
pub fn despawn_hud(mut commands: Commands, query: Query<Entity, With<HudEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
