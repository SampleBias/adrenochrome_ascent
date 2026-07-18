//! Tunable player movement constants (map-cell units / seconds).

/// Collision radius against solid map cells.
pub const PLAYER_RADIUS: f32 = 0.22;

/// Max ground speed while walking.
pub const WALK_SPEED: f32 = 3.4;

/// Max ground speed while sprinting (Shift).
pub const SPRINT_SPEED: f32 = 5.6;

/// Acceleration toward wish speed (Quake/Doom-style).
pub const ACCELERATION: f32 = 14.0;

/// Ground friction coefficient.
pub const FRICTION: f32 = 8.0;

/// Speed below which velocity snaps to zero after friction.
pub const STOP_SPEED: f32 = 1.0;

/// Mouse look sensitivity (radians per pixel).
pub const LOOK_SENSITIVITY: f32 = 0.0024;

/// Keyboard turn rate (radians / sec) for Q/E and arrows.
pub const KEYBOARD_TURN_RATE: f32 = 1.8;

/// Pitch clamp (radians). Used for hand bob / reticle only — yaw drives the raycaster.
pub const PITCH_MIN: f32 = -1.0;
pub const PITCH_MAX: f32 = 1.0;

/// Resting hand overlay anchor / scale (screen UV).
pub const HAND_ANCHOR: (f32, f32) = (0.78, 0.92);
pub const HAND_SCALE: f32 = 1.15;
