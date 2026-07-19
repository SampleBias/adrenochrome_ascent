//! Lightweight state-overlay UI (TODO-005 / Sprint 3 vitals). Full pixel HUD is TODO-033.

use bevy::prelude::*;

use crate::floor_loader::LoadedFloorInfo;
use crate::game::{CurrentFloor, EndingKind, GameState};
use crate::enemy::{BossFight, WardenOverrides};
use crate::player::{
    weapon_stats, Armor, Health, Inventory, Player, WeaponLoadout,
};

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

/// Marker for the vitals / ammo line updated each frame.
#[derive(Component)]
pub struct VitalsHudText;

/// Boss / flood status line (Floor 3).
#[derive(Component)]
pub struct BossHudText;

/// Tiny in-game floor readout + vitals (Sprint 3).
pub fn spawn_ingame_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    floor: Res<CurrentFloor>,
    info: Res<LoadedFloorInfo>,
) {
    let font = title_font(&asset_server, 14.0);
    let vitals_font = title_font(&asset_server, 15.0);
    let title = if info.name.is_empty() {
        format!("FLOOR {} — {}", floor.number, floor.cluster_name())
    } else {
        format!(
            "FLOOR {} — {}\n{}",
            info.number, info.name, info.subtitle
        )
    };
    commands
        .spawn((
            Name::new("InGameHud"),
            MenuUi,
            DespawnOnExit(GameState::InGame),
            Node {
                position_type: PositionType::Absolute,
                top: px(10),
                left: px(12),
                flex_direction: FlexDirection::Column,
                row_gap: px(6),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!(
                    "{title}\n[E] Interact  [LMB/Ctrl] Fire  [1-4] Weapon  [L] Elevator  [TAB] Cursor"
                )),
                font,
                TextColor(Color::srgba(0.85, 0.8, 0.75, 0.85)),
            ));
            parent.spawn((
                VitalsHudText,
                Text::new("HP --  ARM --  AMMO --"),
                vitals_font.clone(),
                TextColor(Color::srgba(0.9, 0.55, 0.45, 0.95)),
            ));
            parent.spawn((
                BossHudText,
                Text::new(""),
                vitals_font,
                TextColor(Color::srgba(0.85, 0.7, 0.35, 0.95)),
            ));
        });
}

/// Refresh health / armor / weapon / ammo readout.
pub fn sync_vitals_hud(
    player: Query<(&Health, &Armor, &Inventory, &WeaponLoadout), With<Player>>,
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
    **text = format!(
        "HP {hp:.0}  ARM {arm:.0}  |  {name}  AMMO {ammo}   (F5/F6/F7 grant)",
        hp = health.current,
        arm = armor.current,
        name = stats.name,
        ammo = ammo,
    );
}

pub fn sync_boss_hud(
    fight: Res<BossFight>,
    warden: Res<WardenOverrides>,
    mut texts: Query<&mut Text, With<BossHudText>>,
) {
    let Ok(mut text) = texts.single_mut() else {
        return;
    };
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
