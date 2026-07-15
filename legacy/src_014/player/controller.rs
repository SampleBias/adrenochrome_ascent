use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use crate::game::constants::{
    CROUCH_HEIGHT, CROUCH_SPEED, LOOK_SENSITIVITY, PLAYER_HEIGHT, SPRINT_SPEED, WALK_SPEED,
};

/// Marker for the player entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct Player;

/// The camera parented to the player (first-person view).
#[derive(Component, Debug, Clone, Copy)]
pub struct PlayerCamera;

/// Runtime movement state.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct PlayerMovement {
    pub yaw: f32,
    pub pitch: f32,
    pub is_sprinting: bool,
    pub is_crouching: bool,
}

/// Captures the mouse on entering a level, releases it on menus.
pub fn capture_mouse(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
}

/// Releases the mouse for menus / pause.
pub fn release_mouse(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

/// Handles keyboard movement (WASD + sprint + crouch) and mouse look.
///
/// This is a simple kinematic controller suitable for a puzzle/stealth game.
/// Physics-based movement (collisions, gravity) is a TODO for when a
/// physics backend (avian / bevy_rapier) is added.
pub fn player_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut PlayerMovement), With<Player>>,
) {
    for (mut transform, mut movement) in query.iter_mut() {
        // --- Mouse look ---
        for event in mouse_motion.read() {
            movement.yaw -= event.delta.x * LOOK_SENSITIVITY;
            movement.pitch -= event.delta.y * LOOK_SENSITIVITY;
        }
        movement.pitch = movement.pitch.clamp(-1.2, 1.2);

        let (sin_yaw, cos_yaw) = movement.yaw.sin_cos();
        // Apply yaw to the player body.
        transform.rotation = Quat::from_rotation_y(movement.yaw);

        // --- Keyboard movement ---
        movement.is_sprinting = keys.pressed(KeyCode::ShiftLeft);
        movement.is_crouching = keys.pressed(KeyCode::KeyC);

        let speed = if movement.is_crouching {
            CROUCH_SPEED
        } else if movement.is_sprinting {
            SPRINT_SPEED
        } else {
            WALK_SPEED
        };

        let mut forward = Vec3::ZERO;
        let mut right = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            forward += Vec3::new(-sin_yaw, 0.0, -cos_yaw);
        }
        if keys.pressed(KeyCode::KeyS) {
            forward -= Vec3::new(-sin_yaw, 0.0, -cos_yaw);
        }
        if keys.pressed(KeyCode::KeyD) {
            right += Vec3::new(cos_yaw, 0.0, -sin_yaw);
        }
        if keys.pressed(KeyCode::KeyA) {
            right -= Vec3::new(cos_yaw, 0.0, -sin_yaw);
        }

        let dir = (forward + right).normalize_or_zero();
        transform.translation += dir * speed * time.delta_seconds();

        // Vertical height (crouch lowers the eye).
        let target_y = if movement.is_crouching {
            CROUCH_HEIGHT
        } else {
            PLAYER_HEIGHT
        };
        // Smoothly interpolate height.
        transform.translation.y = transform.translation.y.lerp(target_y, 0.2);
    }
}

/// Spawns the player + camera at a given position. Called by the level loader.
pub fn spawn_player(commands: &mut Commands, position: Vec3) {
    commands
        .spawn((
            Name::new("Player"),
            Player,
            PlayerMovement::default(),
            Transform::from_translation(position),
            Visibility::Visible,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("PlayerCamera"),
                PlayerCamera,
                Camera3dBundle::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
        });
}

/// Despawns the player. Called when leaving a level.
pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// System set for player systems, for clean ordering.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerSet;
