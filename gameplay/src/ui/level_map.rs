//! Fullscreen level map overlay (M key) with per-floor unlock + facing cone.

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use adrenochrome_engine::{MapGrid, RayCamera};

use crate::floor_loader::LoadedFloorInfo;
use crate::game::{CurrentFloor, GameState};
use crate::interact::InteractionPrompt;
use crate::puzzle::{DnaSequencer, PuzzleRegistry};

use super::TerminalSession;

const LOCKED_FLASH_SECS: f32 = 2.2;
const CELL_PX: f32 = 16.0;

/// In-game map overlay visibility + locked feedback timer.
#[derive(Resource, Debug, Clone)]
pub struct LevelMapState {
    pub open: bool,
    pub locked_flash: f32,
}

impl Default for LevelMapState {
    fn default() -> Self {
        Self {
            open: false,
            locked_flash: 0.0,
        }
    }
}

impl LevelMapState {
    pub fn close(&mut self) {
        self.open = false;
    }
}

/// True while the map is closed — used to freeze player control.
pub fn level_map_not_open(state: Res<LevelMapState>) -> bool {
    !state.open
}

/// Puzzle DSL that unlocks the map for each floor's first progression challenge.
pub fn map_unlock_expr(floor: u8) -> &'static str {
    match floor {
        1 => "has_keycard",
        2 => "has_cell_key",
        3 => "lt_dossier_read",
        4 => "lab_power",
        5 => "email_read",
        6 => "ash_key",
        7 => "warden_valve_a || warden_valve_b",
        8 => "read_scientist_mail || collected_limb >= 1",
        9 => "read_penthouse_memo",
        10 => "dna_manual_started",
        _ => "false",
    }
}

pub fn map_is_unlocked(floor: u8, registry: &PuzzleRegistry) -> bool {
    registry.evaluate(map_unlock_expr(floor))
}

#[derive(Component, Debug, Clone, Copy)]
pub struct LevelMapUi;

fn nes_font(asset_server: &AssetServer, size: f32) -> TextFont {
    TextFont {
        font: FontSource::Handle(asset_server.load("fonts/PressStart2P-Regular.ttf")),
        font_size: FontSize::Px(size),
        ..default()
    }
}

/// M toggles map when unlocked; locked press flashes a HUD prompt. Esc closes map.
pub fn handle_level_map_input(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    floor: Res<CurrentFloor>,
    registry: Res<PuzzleRegistry>,
    dna: Res<DnaSequencer>,
    terminal: Res<TerminalSession>,
    mut map_state: ResMut<LevelMapState>,
    mut prompt: ResMut<InteractionPrompt>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    info: Res<LoadedFloorInfo>,
    grid: Res<MapGrid>,
    camera: Res<RayCamera>,
    existing: Query<Entity, With<LevelMapUi>>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if map_state.locked_flash > 0.0 {
        map_state.locked_flash = (map_state.locked_flash - time.delta_secs()).max(0.0);
        if map_state.locked_flash > 0.0 {
            prompt.text = Some("MAP LOCKED — complete the first challenge".into());
            prompt.blocked = true;
        }
    }

    if dna.active || terminal.active {
        return;
    }

    if keys.just_pressed(KeyCode::Escape) && map_state.open {
        close_map(
            &mut map_state,
            &mut commands,
            &existing,
            &mut cursor,
        );
        return;
    }

    if !keys.just_pressed(KeyCode::KeyM) {
        return;
    }

    if map_state.open {
        close_map(
            &mut map_state,
            &mut commands,
            &existing,
            &mut cursor,
        );
        return;
    }

    if !map_is_unlocked(floor.number, &registry) {
        map_state.locked_flash = LOCKED_FLASH_SECS;
        prompt.text = Some("MAP LOCKED — complete the first challenge".into());
        prompt.blocked = true;
        return;
    }

    map_state.open = true;
    map_state.locked_flash = 0.0;
    if let Ok(mut options) = cursor.single_mut() {
        options.visible = true;
        options.grab_mode = CursorGrabMode::None;
    }
    for entity in &existing {
        commands.entity(entity).despawn();
    }
    spawn_level_map_ui(
        &mut commands,
        &asset_server,
        &info,
        floor.number,
        &grid,
        &camera,
    );
}

fn close_map(
    map_state: &mut LevelMapState,
    commands: &mut Commands,
    existing: &Query<Entity, With<LevelMapUi>>,
    cursor: &mut Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    map_state.close();
    for entity in existing.iter() {
        commands.entity(entity).despawn();
    }
    if let Ok(mut options) = cursor.single_mut() {
        options.visible = false;
        options.grab_mode = CursorGrabMode::Locked;
    }
}

/// Ensure overlay is gone when leaving InGame.
pub fn reset_level_map_on_exit(
    mut map_state: ResMut<LevelMapState>,
    mut commands: Commands,
    existing: Query<Entity, With<LevelMapUi>>,
) {
    map_state.close();
    map_state.locked_flash = 0.0;
    for entity in &existing {
        commands.entity(entity).despawn();
    }
}

fn spawn_level_map_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    info: &LoadedFloorInfo,
    floor_number: u8,
    grid: &MapGrid,
    camera: &RayCamera,
) {
    let title_font = nes_font(asset_server, 12.0);
    let micro = nes_font(asset_server, 7.0);
    let title = if info.name.is_empty() {
        format!("FLOOR {floor_number:02} MAP")
    } else {
        format!("FLOOR {floor_number:02} — {}", info.name.to_uppercase())
    };

    let player_cell = (
        camera.pos.x.floor() as isize,
        camera.pos.y.floor() as isize,
    );
    let lit = flashlight_cells(camera.pos, camera.dir, 3);

    commands
        .spawn((
            Name::new("LevelMapUi"),
            LevelMapUi,
            DespawnOnExit(GameState::InGame),
            Node {
                width: percent(100),
                height: percent(100),
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(14),
                ..default()
            },
            BackgroundColor(Color::srgba(0.02, 0.01, 0.02, 0.82)),
            GlobalZIndex(20),
        ))
        .with_children(|root| {
            root.spawn((
                Text::new(title),
                title_font,
                TextColor(Color::srgb(0.95, 0.22, 0.24)),
            ));

            root.spawn((
                Text::new("YOU  ■   LOOK  ░"),
                micro.clone(),
                TextColor(Color::srgb(0.55, 0.72, 0.68)),
            ));

            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(px(8)),
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.06, 0.04, 0.07)),
                BorderColor::all(Color::srgb(0.45, 0.12, 0.14)),
            ))
            .with_children(|panel| {
                for y in 0..grid.height {
                    panel
                        .spawn((Node {
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },))
                        .with_children(|row| {
                            for x in 0..grid.width {
                                let solid = grid.get(x as isize, y as isize) != 0;
                                let is_player =
                                    x as isize == player_cell.0 && y as isize == player_cell.1;
                                let cone = !is_player
                                    && lit.iter().any(|(lx, ly, _)| *lx == x as isize && *ly == y as isize);
                                let cone_alpha = lit
                                    .iter()
                                    .find(|(lx, ly, _)| *lx == x as isize && *ly == y as isize)
                                    .map(|(_, _, a)| *a)
                                    .unwrap_or(0.0);

                                let color = if is_player {
                                    Color::srgb(0.45, 0.95, 0.85)
                                } else if cone {
                                    Color::srgba(0.95, 0.82, 0.28, 0.35 + cone_alpha * 0.55)
                                } else if solid {
                                    Color::srgb(0.28, 0.08, 0.10)
                                } else {
                                    Color::srgb(0.12, 0.09, 0.12)
                                };

                                row.spawn((
                                    Node {
                                        width: px(CELL_PX),
                                        height: px(CELL_PX),
                                        border: UiRect::all(px(0.5)),
                                        ..default()
                                    },
                                    BackgroundColor(color),
                                    BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.35)),
                                ));
                            }
                        });
                }
            });

            root.spawn((
                Text::new("M / ESC CLOSE"),
                micro,
                TextColor(Color::srgb(0.50, 0.48, 0.52)),
            ));
        });
}

/// Cells lit by a short facing cone (flashlight), with falloff weight in `0..=1`.
fn flashlight_cells(pos: Vec2, dir: Vec2, steps: i32) -> Vec<(isize, isize, f32)> {
    let dir = if dir.length_squared() > 0.0001 {
        dir.normalize()
    } else {
        Vec2::X
    };
    let perp = Vec2::new(-dir.y, dir.x);
    let mut out: Vec<(isize, isize, f32)> = Vec::new();
    for step in 1..=steps {
        let t = step as f32;
        let falloff = 1.0 - (t - 1.0) / steps as f32;
        let center = pos + dir * (t + 0.35);
        let half = ((step as f32) * 0.55).ceil() as i32;
        for side in -half..=half {
            let p = center + perp * side as f32 * 0.55;
            let cx = p.x.floor() as isize;
            let cy = p.y.floor() as isize;
            let side_fade = 1.0 - (side.abs() as f32) / (half as f32 + 1.0);
            let alpha = (falloff * side_fade).clamp(0.15, 1.0);
            if let Some(existing) = out.iter_mut().find(|(x, y, _)| *x == cx && *y == cy) {
                existing.2 = existing.2.max(alpha);
            } else {
                out.push((cx, cy, alpha));
            }
        }
    }
    out
}
