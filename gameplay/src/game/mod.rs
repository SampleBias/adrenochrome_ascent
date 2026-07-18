//! Game flow: states, floors, elevator transitions.

mod floor;
mod states;
mod transitions;

pub use floor::{CurrentFloor, EndingKind};
pub use states::GameState;
pub use transitions::{
    ElevatorTimer, apply_floor_palette, enter_elevator, enter_in_game_spawn_player,
    enter_main_menu_reset, flow_input, release_mouse, request_elevator, tick_elevator,
};
