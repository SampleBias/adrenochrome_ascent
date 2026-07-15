use bevy::prelude::*;

use crate::enemy::components::{PlayerDetected, Scientist};
use crate::game::constants::{
    DETECTION_DECAY, DETECTION_MAX, DETECTION_RATE, ENEMY_VIEW_DISTANCE, ENEMY_VIEW_FOV_COS,
    PATROL_SPEED,
};
use crate::game::plugin::DetectionMeter;
use crate::game::states::GameState;
use crate::player::controller::Player;

/// Moves scientists along their waypoints.
pub fn scientist_patrol(time: Res<Time>, mut query: Query<(&mut Transform, &mut Scientist)>) {
    for (mut transform, mut scientist) in query.iter_mut() {
        if scientist.waypoints.is_empty() {
            continue;
        }

        let target = scientist.waypoints[scientist.current_waypoint];
        let direction = target - transform.translation;
        let distance = direction.length();

        if distance < 0.1 {
            scientist.current_waypoint =
                (scientist.current_waypoint + 1) % scientist.waypoints.len();
        } else {
            let move_dir = direction / distance;
            transform.translation += move_dir * scientist.speed * time.delta_seconds();
            // Face the movement direction.
            if move_dir.length_squared() > 0.0 {
                transform.rotation = Quat::from_rotation_arc(Vec3::Z, move_dir);
            }
        }
    }
}

/// Checks if any scientist can see the player; updates the detection meter.
pub fn detection_check(
    time: Res<Time>,
    state: Res<State<GameState>>,
    detection: ResMut<DetectionMeter>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Scientist>>,
    mut detected_writer: EventWriter<PlayerDetected>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Only run during playable levels.
    if state.get().level_number().is_none() {
        return;
    }

    let mut detection = detection;
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    let mut being_seen = false;
    for enemy_transform in enemy_query.iter() {
        let to_player = player_transform.translation - enemy_transform.translation;
        let dist = to_player.length();

        if dist > ENEMY_VIEW_DISTANCE {
            continue;
        }

        // FOV check: dot product of enemy forward and direction to player.
        let enemy_forward = enemy_transform.rotation * Vec3::Z;
        let dir = to_player / dist;
        if enemy_forward.dot(dir) >= ENEMY_VIEW_FOV_COS {
            being_seen = true;
            break;
        }
    }

    if being_seen {
        detection.value += DETECTION_RATE * time.delta_seconds();
    } else {
        detection.value -= DETECTION_DECAY * time.delta_seconds();
    }
    detection.value = detection.value.clamp(0.0, DETECTION_MAX);

    if detection.is_caught() {
        detected_writer.send(PlayerDetected);
        next_state.set(GameState::GameOver);
    }
}

/// Spawns a patrolling scientist for levels that have one.
/// TODO: call this from per-level setup with bespoke waypoints.
pub fn spawn_scientist(commands: &mut Commands, waypoints: Vec<Vec3>) {
    commands.spawn((
        Name::new("Scientist"),
        Scientist {
            waypoints,
            current_waypoint: 0,
            speed: PATROL_SPEED,
        },
        Transform::default(),
        Visibility::Visible,
    ));
}
