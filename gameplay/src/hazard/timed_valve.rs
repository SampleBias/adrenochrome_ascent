//! Timed valve windows (TODO-022).

use bevy::prelude::*;

use crate::puzzle::PuzzleRegistry;

/// Active timed-valve countdown for a puzzle flag.
#[derive(Resource, Debug, Clone, Default)]
pub struct TimedValveState {
    /// flag → seconds remaining while held true
    pub active: Vec<(String, f32)>,
}

impl TimedValveState {
    pub fn arm(&mut self, flag: impl Into<String>, window_secs: f32) {
        let flag = flag.into();
        if let Some(entry) = self.active.iter_mut().find(|(f, _)| *f == flag) {
            entry.1 = window_secs;
        } else {
            self.active.push((flag, window_secs));
        }
    }
}

pub fn spawn_timed_valve_tracker(mut commands: Commands) {
    commands.insert_resource(TimedValveState::default());
}

/// Called from interact when a TimedValve action fires.
pub fn arm_timed_valve_from_interact(
    flag: &str,
    window_secs: f32,
    valves: &mut TimedValveState,
    registry: &mut PuzzleRegistry,
) {
    registry.set(flag, true);
    valves.arm(flag, window_secs.max(0.5));
    info!("Timed valve '{flag}' open for {window_secs:.1}s");
}

/// Tick windows; clear flags when expired.
pub fn tick_timed_valves(
    time: Res<Time>,
    mut valves: ResMut<TimedValveState>,
    mut registry: ResMut<PuzzleRegistry>,
) {
    let dt = time.delta_secs();
    let mut expired = Vec::new();
    for (flag, remaining) in valves.active.iter_mut() {
        *remaining -= dt;
        if *remaining <= 0.0 {
            expired.push(flag.clone());
        }
    }
    valves.active.retain(|(_, t)| *t > 0.0);
    for flag in expired {
        registry.set(&flag, false);
        info!("Timed valve '{flag}' sealed (window expired)");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm_refreshes_window() {
        let mut state = TimedValveState::default();
        state.arm("v", 2.0);
        state.arm("v", 5.0);
        assert_eq!(state.active.len(), 1);
        assert!((state.active[0].1 - 5.0).abs() < 1e-4);
    }
}
