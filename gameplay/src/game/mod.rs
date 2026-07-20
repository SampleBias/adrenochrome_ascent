//! Game flow: states, floors, elevator transitions, settings.

mod floor;
mod settings;
mod states;
mod transitions;

pub use floor::{resolve_ending_from_flags, CurrentFloor, EndingKind};
pub use settings::{apply_crt_settings, apply_fullscreen_setting, GameSettings};
pub use states::GameState;
pub use transitions::{
    apply_floor_palette, enter_elevator, enter_in_game_spawn_player, enter_main_menu_reset,
    flow_input, release_mouse, request_elevator, set_frame_attract, set_frame_raycast,
    tick_elevator, watch_player_death, ElevatorTimer, MenuCursor, OptionsReturn, SoftInGameResume,
};
