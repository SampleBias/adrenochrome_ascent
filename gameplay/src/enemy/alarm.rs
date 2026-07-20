//! Floor-wide stealth alarm (TODO-028).

use bevy::prelude::*;

use super::components::{EnemyAi, EnemyState, PlayerDetected};

/// Raised by Admin Secretaries (or puzzle/interact) — all hostiles chase.
#[derive(Resource, Debug, Clone, Copy)]
pub struct FloorAlarm {
    pub active: bool,
    /// Seconds remaining before the alarm auto-clears (0 = sticky until floor change).
    pub time_left: f32,
}

impl Default for FloorAlarm {
    fn default() -> Self {
        Self {
            active: false,
            time_left: 0.0,
        }
    }
}

impl FloorAlarm {
    pub fn raise(&mut self, duration: f32) {
        self.active = true;
        self.time_left = duration.max(self.time_left);
        info!("Floor alarm raised ({duration:.0}s)");
    }

    pub fn clear(&mut self) {
        self.active = false;
        self.time_left = 0.0;
    }
}

pub fn reset_floor_alarm(mut alarm: ResMut<FloorAlarm>) {
    alarm.clear();
}

pub fn tick_floor_alarm(time: Res<Time>, mut alarm: ResMut<FloorAlarm>) {
    if !alarm.active || alarm.time_left <= 0.0 {
        return;
    }
    alarm.time_left = (alarm.time_left - time.delta_secs()).max(0.0);
    if alarm.time_left <= 0.0 {
        alarm.clear();
        info!("Floor alarm cleared");
    }
}

/// Admin Secretaries trip the floor alarm when they first spot the player.
pub fn secretary_raise_alarm(
    mut events: MessageReader<PlayerDetected>,
    mut alarm: ResMut<FloorAlarm>,
    ais: Query<&EnemyAi>,
) {
    for ev in events.read() {
        let Ok(ai) = ais.get(ev.enemy) else {
            continue;
        };
        if ai.triggers_alarm {
            alarm.raise(45.0);
        }
    }
}

/// While the alarm is hot, pull every living AI into chase (except flee-types stay fleeing).
pub fn apply_floor_alarm(alarm: Res<FloorAlarm>, mut query: Query<&mut EnemyAi>) {
    if !alarm.active {
        return;
    }
    for mut ai in &mut query {
        if matches!(ai.state, EnemyState::Dead | EnemyState::Stunned) {
            continue;
        }
        if ai.flees {
            if !matches!(ai.state, EnemyState::Flee) {
                ai.state = EnemyState::Flee;
            }
        } else if matches!(ai.state, EnemyState::Patrol) {
            ai.state = EnemyState::Chase;
        }
    }
}
