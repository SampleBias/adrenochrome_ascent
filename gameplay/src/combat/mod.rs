//! Minimal combat targets + hit reactions (TODO-014). Full factions are Sprint 4.

use bevy::prelude::*;

use adrenochrome_engine::{Billboard, RayCamera};

use crate::player::{Player, PlayerMotor, ScreenShake};

/// Anything the hitscan weapons can hurt.
#[derive(Component, Debug, Clone, Copy)]
pub struct CombatTarget {
    pub health: f32,
    pub max_health: f32,
    pub dead: bool,
}

impl Default for CombatTarget {
    fn default() -> Self {
        Self {
            health: 40.0,
            max_health: 40.0,
            dead: false,
        }
    }
}

/// Brief white/red flash on hit.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct HitFlash {
    pub timer: f32,
}

pub fn tick_hit_flash(time: Res<Time>, mut query: Query<&mut HitFlash>) {
    let dt = time.delta_secs();
    for mut flash in &mut query {
        flash.timer = (flash.timer - dt).max(0.0);
    }
}

/// Push hit-flash intensity onto the billboard the raycaster tints.
pub fn sync_hit_flash_visual(mut query: Query<(&HitFlash, &mut Billboard)>) {
    for (flash, mut billboard) in &mut query {
        billboard.flash = if flash.timer > 0.0 {
            (flash.timer / 0.15).clamp(0.0, 1.0)
        } else {
            0.0
        };
    }
}

/// Despawn non-enemy combat props that die (enemies go through loot first).
pub fn despawn_dead_targets(
    mut commands: Commands,
    query: Query<(Entity, &CombatTarget), Without<crate::enemy::Enemy>>,
) {
    for (entity, target) in &query {
        if target.dead {
            commands.entity(entity).despawn();
        }
    }
}

/// Apply screen shake as yaw jitter on top of the motor (after [`player_move`]).
pub fn apply_screen_shake(
    time: Res<Time>,
    shake: Res<ScreenShake>,
    motor: Query<&PlayerMotor, With<Player>>,
    mut camera: ResMut<RayCamera>,
) {
    let Ok(motor) = motor.single() else {
        return;
    };
    if shake.trauma <= 0.0 {
        return;
    }
    let t = time.elapsed_secs() * 40.0;
    let amp = shake.trauma * shake.trauma * 0.08;
    camera.set_yaw(motor.yaw + t.sin() * amp);
}
