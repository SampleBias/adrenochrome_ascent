use bevy::prelude::*;

use crate::enemy::components::PlayerDetected;
use crate::enemy::systems::{detection_check, scientist_patrol};
use crate::game::conditions::in_any_level;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDetected>().add_systems(
            Update,
            (scientist_patrol, detection_check).run_if(in_any_level),
        );
    }
}
