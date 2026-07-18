//! First-person pose for the software raycaster (Doom-style).
//!
//! Position is in map-cell units. `dir` is the facing vector; `plane` is the
//! camera plane that defines FOV (`plane.length()` ≈ tan(fov/2)).
//!
//! The gameplay player controller owns movement and writes this resource each
//! frame before [`crate::RaycasterSystems::Render`].

use bevy::prelude::*;

/// Default horizontal FOV factor (plane length). ~66° like classic Wolf3D/Doom.
pub const DEFAULT_PLANE_LEN: f32 = 0.66;

/// Raycaster view pose.
#[derive(Resource, Debug, Clone, Copy)]
pub struct RayCamera {
    pub pos: Vec2,
    pub dir: Vec2,
    pub plane: Vec2,
}

impl Default for RayCamera {
    fn default() -> Self {
        Self::from_yaw(Vec2::new(1.5, 1.5), 0.0)
    }
}

impl RayCamera {
    /// Build a camera at `pos` facing `yaw` radians (0 = +X, CCW toward +Y).
    pub fn from_yaw(pos: Vec2, yaw: f32) -> Self {
        let (s, c) = yaw.sin_cos();
        let dir = Vec2::new(c, s);
        // Plane is perpendicular to dir, length sets FOV.
        let plane = Vec2::new(-dir.y, dir.x) * DEFAULT_PLANE_LEN;
        Self { pos, dir, plane }
    }

    pub fn yaw(&self) -> f32 {
        self.dir.y.atan2(self.dir.x)
    }

    pub fn set_yaw(&mut self, yaw: f32) {
        let (s, c) = yaw.sin_cos();
        self.dir = Vec2::new(c, s);
        let len = self.plane.length().max(0.01);
        self.plane = Vec2::new(-self.dir.y, self.dir.x) * len;
    }

    pub fn rotate(&mut self, delta_yaw: f32) {
        self.set_yaw(self.yaw() + delta_yaw);
    }
}
