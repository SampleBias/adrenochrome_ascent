//! State overlays: main menu, options, credits, game over, ending (TODO-005 / TODO-040).

use bevy::prelude::*;

use crate::game::{CurrentFloor, EndingKind, GameSettings, GameState, MenuCursor};
use crate::enemy::{BossFight, ScientistFight, WardenOverrides};
use crate::player::{
    weapon_stats, Armor, Health, Inventory, Player, SerumEffect, WeaponLoadout,
};
use crate::puzzle::{DnaHudText, PuzzleRegistry};

/// Marker for state overlay UI roots (despawned on state exit).
#[derive(Component, Debug, Clone, Copy)]
pub struct MenuUi;

/// Blinking "PUSH START BUTTON" line on the NES title screen.
#[derive(Component, Debug, Clone, Copy)]
pub struct PushStartBlink;

fn title_font(asset_server: &AssetServer, size: f32) -> TextFont {
    TextFont {
        font: FontSource::Handle(asset_server.load("fonts/FiraSans-Bold.ttf")),
        font_size: FontSize::Px(size),
        ..default()
    }
}

/// Late-80s NES pixel face (Press Start 2P).
fn nes_font(asset_server: &AssetServer, size: f32) -> TextFont {
    TextFont {
        font: FontSource::Handle(asset_server.load("fonts/PressStart2P-Regular.ttf")),
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

/// NES cartridge title screen — transparent overlay over the attract CRT backdrop.
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let brand = nes_font(&asset_server, 22.0);
    let tag = nes_font(&asset_server, 8.0);
    let body = nes_font(&asset_server, 11.0);
    let micro = nes_font(&asset_server, 7.0);

    commands
        .spawn((
            Name::new("MainMenuUi"),
            MenuUi,
            DespawnOnExit(GameState::MainMenu),
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                padding: UiRect::axes(px(24), px(28)),
                ..default()
            },
            // Let the attract framebuffer read through (no solid chrome plate).
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.28)),
        ))
        .with_children(|parent| {
            // Top: license-line flavor + brand block.
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: px(10),
                        margin: UiRect::top(px(18)),
                        ..default()
                    },
                ))
                .with_children(|top| {
                    top.spawn((
                        Text::new("ADRENOCHROME"),
                        brand.clone(),
                        TextColor(Color::srgb(0.95, 0.12, 0.16)),
                    ));
                    top.spawn((
                        Text::new("ASCENT"),
                        brand,
                        TextColor(Color::srgb(0.98, 0.98, 0.92)),
                    ));
                    top.spawn((
                        Text::new("- ESCAPE TO SURVIVE -"),
                        tag.clone(),
                        TextColor(Color::srgb(0.45, 0.85, 0.75)),
                    ));
                });

            // Mid: menu rows with NES cursor.
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        row_gap: px(8),
                        ..default()
                    },
                ))
                .with_children(|mid| {
                    mid.spawn((
                        MainMenuBody,
                        Text::new(menu_body_text(0, 1)),
                        body,
                        TextColor(Color::srgb(0.92, 0.90, 0.88)),
                    ));
                });

            // Bottom: PUSH START blink + copyright strip.
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: px(10),
                        margin: UiRect::bottom(px(8)),
                        ..default()
                    },
                ))
                .with_children(|bot| {
                    bot.spawn((
                        PushStartBlink,
                        Text::new("PUSH START BUTTON"),
                        tag.clone(),
                        TextColor(Color::srgb(0.98, 0.98, 0.85)),
                    ));
                    bot.spawn((
                        Text::new("2026 SYNLBS Licensed to Horror"),
                        micro.clone(),
                        TextColor(Color::srgb(0.55, 0.52, 0.58)),
                    ));
                    bot.spawn((
                        Text::new("UD SELECT   START CONFIRM   B QUIT"),
                        micro,
                        TextColor(Color::srgb(0.40, 0.38, 0.42)),
                    ));
                });
        });
}

#[derive(Component)]
pub struct MainMenuBody;

fn menu_body_text(index: usize, load_slot: u8) -> String {
    let rows = [
        "NEW GAME".to_string(),
        format!("CONTINUE  <SLOT {load_slot:02}>"),
        "OPTIONS".to_string(),
        "CREDITS".to_string(),
        "QUIT".to_string(),
    ];
    let mut out = String::new();
    for (i, row) in rows.iter().enumerate() {
        if i == index {
            // ASCII cursor — Press Start 2P has no ▶ glyph.
            out.push_str(&format!("> {row}\n"));
        } else {
            out.push_str(&format!("  {row}\n"));
        }
    }
    out
}

pub fn sync_main_menu_cursor(
    cursor: Res<MenuCursor>,
    mut texts: Query<&mut Text, With<MainMenuBody>>,
) {
    if !cursor.is_changed() {
        return;
    }
    let Ok(mut text) = texts.single_mut() else {
        return;
    };
    **text = menu_body_text(cursor.index, cursor.load_slot);
}

/// Classic NES title blink (~2 Hz).
pub fn blink_push_start(
    time: Res<Time>,
    mut q: Query<&mut Visibility, With<PushStartBlink>>,
) {
    let visible = (time.elapsed_secs() * 2.0).fract() < 0.55;
    for mut vis in &mut q {
        *vis = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn spawn_options_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    let title = title_font(&asset_server, 28.0);
    let body = title_font(&asset_server, 16.0);
    commands
        .spawn((
            Name::new("OptionsUi"),
            MenuUi,
            DespawnOnExit(GameState::Options),
            fullscreen_root(Color::srgba(0.02, 0.02, 0.04, 0.9)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("OPTIONS"),
                title,
                TextColor(Color::srgb(0.7, 0.85, 0.7)),
            ));
            parent.spawn((
                OptionsBody,
                Text::new(options_body(&settings)),
                body,
                TextColor(Color::srgb(0.75, 0.75, 0.78)),
            ));
        });
}

#[derive(Component)]
pub struct OptionsBody;

fn options_body(s: &GameSettings) -> String {
    format!(
        "[M] Music volume   {:.0}%\n\
         [N] SFX volume     {:.0}%\n\
         [C] CRT effects    {}\n\
         [V] Dither         {}\n\
         [F] Fullscreen     {}\n\n\
         [ENTER/ESC] Back",
        s.music_volume * 100.0,
        s.sfx_volume * 100.0,
        if s.crt_enabled { "ON" } else { "OFF" },
        if s.dither_enabled { "ON" } else { "OFF" },
        if s.fullscreen { "ON" } else { "OFF" },
    )
}

pub fn sync_options_body(
    settings: Res<GameSettings>,
    mut texts: Query<&mut Text, With<OptionsBody>>,
) {
    if !settings.is_changed() {
        return;
    }
    let Ok(mut text) = texts.single_mut() else {
        return;
    };
    **text = options_body(&settings);
}

pub fn spawn_credits(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title = title_font(&asset_server, 28.0);
    let body = title_font(&asset_server, 15.0);
    commands
        .spawn((
            Name::new("CreditsUi"),
            MenuUi,
            DespawnOnExit(GameState::Credits),
            fullscreen_root(Color::srgba(0.01, 0.01, 0.02, 0.92)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("CREDITS"),
                title,
                TextColor(Color::srgb(0.85, 0.75, 0.55)),
            ));
            parent.spawn((
                Text::new(
                    "Adrenochrome Ascent\n\
                     Engine · Gameplay · Content\n\
                     Bevy 0.19 software raycaster + CRT\n\n\
                     Thanks for ascending.\n\n\
                     [ENTER/ESC] Back",
                ),
                body,
                TextColor(Color::srgb(0.7, 0.7, 0.72)),
            ));
        });
}

pub fn spawn_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    let title = title_font(&asset_server, 34.0);
    let body = title_font(&asset_server, 16.0);
    commands
        .spawn((
            Name::new("GameOverUi"),
            MenuUi,
            DespawnOnExit(GameState::GameOver),
            fullscreen_root(Color::srgba(0.08, 0.0, 0.0, 0.88)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("YOU DIED"),
                title,
                TextColor(Color::srgb(0.85, 0.2, 0.2)),
            ));
            parent.spawn((
                Text::new("[ENTER] Load last autosave    [ESC] Main Menu"),
                body,
                TextColor(Color::srgb(0.8, 0.7, 0.7)),
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
    let road = match *ending {
        EndingKind::Released => {
            "Snow. Headlights. Open gates.\nLimos idle on the mountain road —\nand someone else gets daylight."
        }
        EndingKind::Contained => {
            "The convoy swallows the mountain road.\nEngines. Black glass. No witnesses.\nWhatever you left below stays below."
        }
    };

    commands
        .spawn((
            Name::new("EndingUi"),
            MenuUi,
            DespawnOnExit(GameState::Ending),
            fullscreen_root(Color::srgba(0.01, 0.02, 0.03, 0.82)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(ending.title()),
                title,
                TextColor(Color::srgb(0.85, 0.75, 0.55)),
            ));
            parent.spawn((
                Text::new(format!(
                    "{}\n\n{}\n\n[ENTER] Main Menu",
                    road,
                    ending.blurb()
                )),
                body,
                TextColor(Color::srgb(0.7, 0.7, 0.72)),
            ));
        });
}

/// Marker for the vitals / ammo line updated each frame.
#[derive(Component)]
pub struct VitalsHudText;

/// Boss / flood status line (Floor 3).
#[derive(Component)]
pub struct BossHudText;

/// Minimal help overlay — vitals/PA render in the CRT pixel HUD (TODO-033).
pub fn spawn_ingame_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = title_font(&asset_server, 13.0);
    commands
        .spawn((
            Name::new("InGameHelp"),
            MenuUi,
            DespawnOnExit(GameState::InGame),
            Node {
                position_type: PositionType::Absolute,
                bottom: px(8),
                right: px(10),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("[E] Interact  [1-4] Weapon  [ESC] Options"),
                font,
                TextColor(Color::srgba(0.75, 0.72, 0.68, 0.55)),
            ));
            parent.spawn((VitalsHudText, Text::new(""), font_size_zero(&asset_server)));
            parent.spawn((BossHudText, Text::new(""), font_size_zero(&asset_server)));
            parent.spawn((DnaHudText, Text::new(""), font_size_zero(&asset_server)));
        });
}

fn font_size_zero(asset_server: &AssetServer) -> TextFont {
    TextFont {
        font: FontSource::Handle(asset_server.load("fonts/FiraSans-Bold.ttf")),
        font_size: FontSize::Px(1.0),
        ..default()
    }
}

/// Refresh health / armor / weapon / ammo readout (legacy markers; pixel HUD is primary).
pub fn sync_vitals_hud(
    player: Query<(&Health, &Armor, &Inventory, &WeaponLoadout), With<Player>>,
    serum: Res<SerumEffect>,
    registry: Res<PuzzleRegistry>,
    mut texts: Query<&mut Text, With<VitalsHudText>>,
) {
    let Ok((health, armor, inv, loadout)) = player.single() else {
        return;
    };
    let Ok(mut text) = texts.single_mut() else {
        return;
    };
    let stats = weapon_stats(loadout.current);
    let ammo = inv.ammo_for(stats.ammo);
    let serum_tag = if serum.active { "  SERUM!" } else { "" };
    let limbs = registry.counter("collected_limb");
    **text = format!(
        "HP {hp:.0}  ARM {arm:.0}  |  {name}  AMMO {ammo}  limbs {limbs}{serum_tag}",
        hp = health.current,
        arm = armor.current,
        name = stats.name,
        ammo = ammo,
        limbs = limbs,
        serum_tag = serum_tag,
    );
}

pub fn sync_boss_hud(
    fight: Res<BossFight>,
    warden: Res<WardenOverrides>,
    scientist: Res<ScientistFight>,
    mut texts: Query<&mut Text, With<BossHudText>>,
) {
    let Ok(mut text) = texts.single_mut() else {
        return;
    };
    if scientist.active || scientist.defeated {
        if scientist.defeated {
            **text = "SCIENTIST DOWN — research cleared".into();
            return;
        }
        **text = format!(
            "SCIENTIST  phase {}  |  DNA when stunned — Injector cures serum",
            scientist.phase
        );
        return;
    }
    if warden.active || warden.defeated {
        if warden.defeated {
            **text = "WARDEN DOWN — security cleared".into();
            return;
        }
        let pause = if warden.combat_paused {
            "VALVE PHASE — turn coolant / drain"
        } else {
            "ENGAGED"
        };
        **text = format!(
            "WARDEN  phase {}  flood {:.0}%  |  {pause}",
            warden.phase,
            warden.flood_level * 100.0
        );
        return;
    }
    if !fight.active && !fight.defeated {
        **text = String::new();
        return;
    }
    if fight.defeated {
        **text = "LIEUTENANT DOWN — vault path clear".into();
        return;
    }
    let cigar = if fight.cigar_vulnerable {
        "CIGAR LIT — SHOOT"
    } else {
        "cigar dark"
    };
    **text = format!(
        "LIEUTENANT  phase {}  flood {:.0}%  |  {cigar}",
        fight.phase,
        fight.flood_level * 100.0
    );
}
