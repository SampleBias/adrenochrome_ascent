//! Environmental hazards: timed valves + grid crate pushing (TODO-022).

mod crate_push;
mod timed_valve;

pub use crate_push::{push_crates, spawn_crate, PushableCrate};
pub use timed_valve::{
    arm_timed_valve_from_interact, spawn_timed_valve_tracker, tick_timed_valves, TimedValveState,
};
