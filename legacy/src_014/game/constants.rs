//! Tunable gameplay constants.
//!
//! Centralised here so designers can balance the game without hunting
//! through systems. All units are Bevy defaults (meters, radians, seconds).

// Player movement.
pub const PLAYER_HEIGHT: f32 = 1.7;
pub const PLAYER_RADIUS: f32 = 0.3;
pub const WALK_SPEED: f32 = 3.5;
pub const SPRINT_SPEED: f32 = 6.0;
pub const CROUCH_SPEED: f32 = 1.6;
pub const CROUCH_HEIGHT: f32 = 1.0;

/// Mouse look.
pub const LOOK_SENSITIVITY: f32 = 0.0025;

/// Interaction.
pub const INTERACT_DISTANCE: f32 = 2.5;

/// Detection.
/// When `detection` reaches `DETECTION_MAX` the player is caught.
pub const DETECTION_MAX: f32 = 100.0;
pub const DETECTION_RATE: f32 = 25.0; // per second when seen
pub const DETECTION_DECAY: f32 = 15.0; // per second when hidden

/// Enemy patrol.
pub const PATROL_SPEED: f32 = 1.2;
pub const ENEMY_VIEW_DISTANCE: f32 = 8.0;
pub const ENEMY_VIEW_FOV_COS: f32 = 0.5; // ~60° half-angle

/// World.
pub const FLOOR_SIZE: f32 = 20.0;
pub const WALL_HEIGHT: f32 = 3.0;

/// Colors (linear-ish for Bevy 0.14).
pub const FLOOR_COLOR: [f32; 3] = [0.08, 0.08, 0.09];
pub const WALL_COLOR: [f32; 3] = [0.12, 0.11, 0.10];
pub const ACCENT_COLOR: [f32; 3] = [0.6, 0.1, 0.1]; // blood red
