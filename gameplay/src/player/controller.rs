//! Doom-style first-person controller on the raycaster map grid.
//!
//! - Movement is 2D (map X/Y) with acceleration + friction.
//! - Collision is grid-cell based via [`MapGrid::try_move`].
//! - Mouse yaw updates [`RayCamera`]; pitch only nudges the hand overlay.

use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use adrenochrome_engine::{MapGrid, RayCamera};

use super::constants::*;
use super::perks::MutationPerks;
use super::vitals::{Armor, Health, Inventory};
use super::weapons::WeaponLoadout;

/// Marker for the player entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

/// Runtime motor state synced to [`RayCamera`] each frame.
#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerMotor {
    pub pos: Vec2,
    pub yaw: f32,
    /// Look pitch (radians). Does not tilt the raycaster; drives hand/reticle.
    pub pitch: f32,
    pub velocity: Vec2,
    pub is_sprinting: bool,
}

impl PlayerMotor {
    pub fn from_camera(camera: &RayCamera) -> Self {
        Self {
            pos: camera.pos,
            yaw: camera.yaw(),
            pitch: 0.0,
            velocity: Vec2::ZERO,
            is_sprinting: false,
        }
    }
}

/// System set for ordering player logic before the raycaster render.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerSet {
    Input,
    Move,
    Present,
}

/// Spawn the player from the current [`RayCamera`] (placed by the test map / floor loader).
pub fn spawn_player(mut commands: Commands, camera: Res<RayCamera>) {
    commands.spawn((
        Name::new("Player"),
        Player,
        PlayerMotor::from_camera(&camera),
        Health::default(),
        Armor::default(),
        Inventory::default(),
        WeaponLoadout::default(),
    ));
}

/// Lock and hide the cursor for mouse look.
pub fn capture_mouse(mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut options) = cursor.single_mut() {
        options.visible = false;
        options.grab_mode = CursorGrabMode::Locked;
    }
}

/// Show and free the cursor (menus / pause — TODO-005).
pub fn release_mouse(mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut options) = cursor.single_mut() {
        options.visible = true;
        options.grab_mode = CursorGrabMode::None;
    }
}

/// Tab toggles cursor grab so the window stays usable during development.
pub fn toggle_cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if !keys.just_pressed(KeyCode::Tab) {
        return;
    }
    let Ok(mut options) = cursor.single_mut() else {
        return;
    };
    if options.grab_mode == CursorGrabMode::Locked {
        options.visible = true;
        options.grab_mode = CursorGrabMode::None;
    } else {
        options.visible = false;
        options.grab_mode = CursorGrabMode::Locked;
    }
}

/// Mouse + keyboard look. Yaw is authoritative for the raycaster facing.
pub fn player_look(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<AccumulatedMouseMotion>,
    cursor: Query<&CursorOptions, With<PrimaryWindow>>,
    mut query: Query<&mut PlayerMotor, With<Player>>,
) {
    let Ok(mut motor) = query.single_mut() else {
        return;
    };

    // Only apply mouse look while the cursor is captured.
    let grabbed = cursor
        .single()
        .map(|c| c.grab_mode == CursorGrabMode::Locked)
        .unwrap_or(false);

    if grabbed {
        motor.yaw -= mouse.delta.x * LOOK_SENSITIVITY;
        motor.pitch -= mouse.delta.y * LOOK_SENSITIVITY;
    }

    if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyQ) {
        motor.yaw += KEYBOARD_TURN_RATE * time.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyE) {
        motor.yaw -= KEYBOARD_TURN_RATE * time.delta_secs();
    }

    motor.pitch = motor.pitch.clamp(PITCH_MIN, PITCH_MAX);
}

/// WASD movement with Doom-style accelerate / friction and grid collision.
pub fn player_move(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    map: Res<MapGrid>,
    perks: Res<MutationPerks>,
    mut camera: ResMut<RayCamera>,
    mut query: Query<&mut PlayerMotor, With<Player>>,
) {
    let Ok(mut motor) = query.single_mut() else {
        return;
    };

    let dt = time.delta_secs().max(1e-6);
    let (sin_yaw, cos_yaw) = motor.yaw.sin_cos();
    let forward = Vec2::new(cos_yaw, sin_yaw);
    let right = Vec2::new(-sin_yaw, cos_yaw);

    let mut wish = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        wish += forward;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        wish -= forward;
    }
    if keys.pressed(KeyCode::KeyA) {
        wish -= right;
    }
    if keys.pressed(KeyCode::KeyD) {
        wish += right;
    }

    motor.is_sprinting = keys.pressed(KeyCode::ShiftLeft);
    let perk_mul = perks.speed_multiplier();
    let max_speed = if motor.is_sprinting {
        SPRINT_SPEED
    } else {
        WALK_SPEED
    } * perk_mul;

    // Ground friction (always applied — classic feel when releasing keys).
    apply_friction(&mut motor.velocity, dt);

    if wish.length_squared() > 0.0 {
        let wish_dir = wish.normalize();
        accelerate(&mut motor.velocity, wish_dir, max_speed, ACCELERATION, dt);
        // Hard cap so sprint/walk ceilings stay honest after accel overshoot.
        let speed = motor.velocity.length();
        if speed > max_speed {
            motor.velocity *= max_speed / speed;
        }
    }

    let delta = motor.velocity * dt;
    let prev = motor.pos;
    motor.pos = map.try_move(motor.pos, delta, PLAYER_RADIUS);

    // Zero velocity on axes that hit a wall so slides don't feel sticky.
    if (motor.pos.x - prev.x).abs() < 1e-6 {
        motor.velocity.x = 0.0;
    }
    if (motor.pos.y - prev.y).abs() < 1e-6 {
        motor.velocity.y = 0.0;
    }

    camera.pos = motor.pos;
    camera.set_yaw(motor.yaw);
}

/// Apply friction to velocity (Quake-style).
fn apply_friction(velocity: &mut Vec2, dt: f32) {
    let speed = velocity.length();
    if speed < 1e-4 {
        *velocity = Vec2::ZERO;
        return;
    }
    let control = speed.max(STOP_SPEED);
    let drop = control * FRICTION * dt;
    let new_speed = (speed - drop).max(0.0);
    *velocity *= new_speed / speed;
}

/// Accelerate velocity toward `wish_dir * wish_speed`.
fn accelerate(velocity: &mut Vec2, wish_dir: Vec2, wish_speed: f32, accel: f32, dt: f32) {
    let current_speed = velocity.dot(wish_dir);
    let add_speed = wish_speed - current_speed;
    if add_speed <= 0.0 {
        return;
    }
    let accel_speed = (accel * dt * wish_speed).min(add_speed);
    *velocity += wish_dir * accel_speed;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn friction_slows_and_stops() {
        let mut v = Vec2::new(3.0, 0.0);
        apply_friction(&mut v, 0.05);
        assert!(v.x < 3.0 && v.x > 0.0);
        for _ in 0..40 {
            apply_friction(&mut v, 0.05);
        }
        assert_eq!(v, Vec2::ZERO);
    }

    #[test]
    fn accelerate_builds_speed_along_wish() {
        let mut v = Vec2::ZERO;
        let dir = Vec2::new(1.0, 0.0);
        accelerate(&mut v, dir, 3.0, 10.0, 0.1);
        assert!(v.x > 0.0);
        assert!(v.x <= 3.0 + 1e-3);
    }
}
