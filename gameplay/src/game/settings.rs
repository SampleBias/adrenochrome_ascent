//! Player-facing settings (TODO-040).

use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode};
use serde::{Deserialize, Serialize};

use adrenochrome_engine::{
    CrtMaterial, DEFAULT_DITHER, DEFAULT_PHOSPHOR, DEFAULT_SCANLINE, DEFAULT_VIGNETTE,
};

/// Global options (TODO-040).
#[derive(Resource, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GameSettings {
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub crt_enabled: bool,
    pub dither_enabled: bool,
    pub fullscreen: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            music_volume: 0.28,
            sfx_volume: 0.55,
            crt_enabled: true,
            dither_enabled: true,
            fullscreen: false,
        }
    }
}

/// Apply CRT toggles from options into the upscale material.
pub fn apply_crt_settings(
    settings: Res<GameSettings>,
    mut materials: ResMut<Assets<CrtMaterial>>,
) {
    if !settings.is_changed() {
        return;
    }
    for (_, mat) in materials.iter_mut() {
        if settings.crt_enabled {
            mat.crt_params.x = DEFAULT_SCANLINE;
            mat.crt_params.y = DEFAULT_VIGNETTE;
            mat.crt_params.z = if settings.dither_enabled {
                DEFAULT_DITHER
            } else {
                0.0
            };
            mat.post_fx.z = DEFAULT_PHOSPHOR;
        } else {
            mat.crt_params.x = 0.0;
            mat.crt_params.y = 0.15;
            mat.crt_params.z = 0.0;
            mat.post_fx.z = 0.0;
        }
    }
}

/// Toggle window mode from options.
pub fn apply_fullscreen_setting(
    settings: Res<GameSettings>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !settings.is_changed() {
        return;
    }
    let Ok(mut window) = windows.single_mut() else {
        return;
    };
    window.mode = if settings.fullscreen {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
}
