use bevy::prelude::*;

/// A patrolling scientist enemy.
#[derive(Component, Debug, Clone)]
pub struct Scientist {
    /// Patrol waypoints in world space.
    pub waypoints: Vec<Vec3>,
    /// Current waypoint index.
    pub current_waypoint: usize,
    /// Movement speed.
    pub speed: f32,
}

/// A security camera that sweeps an area.
#[derive(Component, Debug, Clone)]
pub struct SecurityCamera {
    pub base_yaw: f32,
    pub sweep_angle: f32,
    pub sweep_speed: f32,
    pub elapsed: f32,
}

/// Event fired when the player is detected (caught).
#[derive(Event, Debug, Clone)]
pub struct PlayerDetected;
