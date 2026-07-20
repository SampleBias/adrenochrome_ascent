//! Hand / viewmodel state machine (TODO-011).

use bevy::prelude::*;

use adrenochrome_engine::HandOverlay;

use super::constants::{HAND_ANCHOR, HAND_SCALE, PITCH_MAX};
use super::perks::MutationPerks;
use super::vitals::PainFlash;
use super::weapons::{weapon_stats, AdrenoVision, MuzzleFlash, WeaponLoadout};
use crate::interact::InteractionPrompt;
use crate::player::PlayerMotor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandState {
    Idle,
    InteractGlow,
    Fire,
}

/// Drive hand texture / glow / muzzle from weapon + interact + fire anim.
pub fn update_hand_viewmodel(
    prompt: Res<InteractionPrompt>,
    muzzle: Res<MuzzleFlash>,
    vision: Res<AdrenoVision>,
    perks: Res<MutationPerks>,
    loadout: Query<&WeaponLoadout>,
    motor: Query<&PlayerMotor>,
    mut hands: Query<&mut HandOverlay>,
) {
    let night = vision.active || perks.night_vision;
    let Ok(loadout) = loadout.single() else {
        return;
    };
    let Ok(motor) = motor.single() else {
        return;
    };
    let stats = weapon_stats(loadout.current);

    let state = if loadout.fire_anim > 0.0 || muzzle.timer > 0.0 {
        HandState::Fire
    } else if prompt.text.is_some() && !prompt.blocked {
        HandState::InteractGlow
    } else {
        HandState::Idle
    };

    for mut hand in &mut hands {
        hand.texture_id = match state {
            HandState::Fire => stats.fire_texture,
            _ => stats.hand_texture,
        };
        hand.glow_pulse = match state {
            HandState::InteractGlow => 6.0,
            HandState::Fire => 10.0,
            HandState::Idle if night => 4.0,
            HandState::Idle => 2.0,
        };
        hand.muzzle = (muzzle.timer / 0.1).clamp(0.0, 1.0);

        let pitch_norm = (motor.pitch / PITCH_MAX).clamp(-1.0, 1.0);
        let fire_kick = loadout.fire_anim * 0.08;
        hand.anchor.x = HAND_ANCHOR.0 + pitch_norm * 0.02;
        hand.anchor.y = HAND_ANCHOR.1 - pitch_norm * 0.06 + fire_kick;
        hand.scale = HAND_SCALE
            * (1.0
                + pitch_norm * 0.04
                + if state == HandState::Fire {
                    0.06
                } else {
                    0.0
                });
        if night {
            hand.scale *= 1.05;
        }
    }
}

/// Fullscreen red pain overlay (UI layer over CRT).
#[derive(Component)]
pub struct PainFlashUi;

pub fn sync_pain_flash_ui(
    pain: Res<PainFlash>,
    mut commands: Commands,
    mut existing: Query<(Entity, &mut BackgroundColor), With<PainFlashUi>>,
) {
    if let Ok((entity, mut bg)) = existing.single_mut() {
        if pain.intensity <= 0.01 {
            commands.entity(entity).despawn();
        } else {
            *bg = BackgroundColor(Color::srgba(0.7, 0.05, 0.08, pain.intensity * 0.45));
        }
        return;
    }
    if pain.intensity <= 0.01 {
        return;
    }
    commands.spawn((
        PainFlashUi,
        Name::new("PainFlash"),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        BackgroundColor(Color::srgba(0.7, 0.05, 0.08, pain.intensity * 0.45)),
        GlobalZIndex(100),
    ));
}
