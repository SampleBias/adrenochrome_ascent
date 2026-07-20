//! Adrenochrome Ascent — Gameplay crate.
//!
//! Player controller, game state flow, floor loading, puzzles, interaction, saves, combat.

use bevy::prelude::*;

use adrenochrome_engine::RaycasterSystems;

pub mod audio;
pub mod combat;
pub mod enemy;
pub mod floor_loader;
pub mod game;
pub mod hazard;
pub mod interact;
pub mod player;
pub mod puzzle;
pub mod save;
pub mod ui;

pub use game::{CurrentFloor, EndingKind, GameState};
pub use player::{Player, PlayerMotor, PlayerSet};
pub use puzzle::PuzzleRegistry;

/// Gameplay plugin: state machine, floors, puzzles, interaction, autosave, combat.
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_resource::<CurrentFloor>()
            .init_resource::<EndingKind>()
            .init_resource::<game::ElevatorTimer>()
            .init_resource::<game::GameSettings>()
            .init_resource::<game::MenuCursor>()
            .init_resource::<game::OptionsReturn>()
            .init_resource::<game::SoftInGameResume>()
            .init_resource::<PuzzleRegistry>()
            .init_resource::<interact::InteractionPrompt>()
            .init_resource::<save::ActiveSaveSlot>()
            .init_resource::<save::PendingLoad>()
            .init_resource::<player::PainFlash>()
            .init_resource::<player::AdrenoVision>()
            .init_resource::<player::ScreenShake>()
            .init_resource::<player::MuzzleFlash>()
            .init_resource::<player::SerumEffect>()
            .init_resource::<enemy::BossFight>()
            .init_resource::<enemy::WardenOverrides>()
            .init_resource::<enemy::ScientistFight>()
            .init_resource::<enemy::FactionRegistry>()
            .init_resource::<enemy::FloorAlarm>()
            .init_resource::<player::MutationPerks>()
            .init_resource::<hazard::TimedValveState>()
            .init_resource::<puzzle::DnaSequencer>()
            .init_resource::<floor_loader::ActiveWaveTuning>()
            .init_resource::<floor_loader::SkipFloorLoad>()
            .add_plugins((audio::GameAudioPlugin, ui::TerminalUiPlugin))
            .add_message::<interact::InteractAttempt>()
            .add_message::<enemy::PlayerDetected>()
            .configure_sets(
                Update,
                (
                    PlayerSet::Input,
                    PlayerSet::Move,
                    PlayerSet::Present,
                )
                    .chain()
                    .before(RaycasterSystems::Render)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(Startup, floor_loader::setup_world_shell)
            // --- MainMenu ---
            .add_systems(
                OnEnter(GameState::MainMenu),
                (
                    floor_loader::unload_floor,
                    game::enter_main_menu_reset,
                    game::release_mouse,
                    ui::spawn_main_menu,
                ),
            )
            // --- InGame ---
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    (
                        enemy::reset_floor_alarm,
                        floor_loader::begin_ingame_enter,
                        floor_loader::load_current_floor,
                        floor_loader::floor9_limb_failsafe,
                        save::apply_pending_load,
                        game::enter_in_game_spawn_player,
                    )
                        .chain(),
                    (
                        player::grant_mutation_perks,
                        player::capture_mouse,
                        ui::spawn_ingame_hud,
                        audio::start_floor_audio,
                    )
                        .chain(),
                )
                    .chain(),
            )
            .add_systems(
                OnExit(GameState::InGame),
                (game::release_mouse, ui::disable_pixel_hud),
            )
            // --- Elevator: autosave current floor, then ride ---
            .add_systems(
                OnEnter(GameState::ElevatorTransition),
                (
                    save::autosave_on_elevator,
                    game::enter_elevator,
                    game::release_mouse,
                    ui::spawn_elevator_overlay,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                game::tick_elevator.run_if(in_state(GameState::ElevatorTransition)),
            )
            // --- Options / Credits / GameOver ---
            .add_systems(
                OnEnter(GameState::Options),
                (game::release_mouse, ui::spawn_options_menu),
            )
            .add_systems(OnEnter(GameState::Credits), ui::spawn_credits)
            .add_systems(
                OnEnter(GameState::GameOver),
                (game::release_mouse, ui::spawn_game_over),
            )
            // --- Ending cinematic (outdoor road + text) ---
            .add_systems(
                OnEnter(GameState::Ending),
                (
                    game::resolve_ending_from_flags,
                    floor_loader::load_ending_cinematic,
                    game::release_mouse,
                    ui::spawn_ending,
                )
                    .chain(),
            )
            .add_systems(Update, game::flow_input)
            .add_systems(
                Update,
                (
                    ui::sync_main_menu_cursor.run_if(in_state(GameState::MainMenu)),
                    ui::sync_options_body.run_if(in_state(GameState::Options)),
                    game::apply_crt_settings,
                    game::apply_fullscreen_setting,
                    game::watch_player_death.run_if(in_state(GameState::InGame)),
                ),
            )
            .add_systems(
                Update,
                (
                    player::toggle_cursor_grab.in_set(PlayerSet::Input),
                    player::player_look.in_set(PlayerSet::Input),
                    player::select_weapon.in_set(PlayerSet::Input),
                    player::debug_grant_weapons.in_set(PlayerSet::Input),
                    player::fire_weapon.in_set(PlayerSet::Input),
                    player::player_move.in_set(PlayerSet::Move),
                    hazard::push_crates.in_set(PlayerSet::Move),
                    player::tick_weapon_timers.in_set(PlayerSet::Present),
                    player::tick_adreno_vision.in_set(PlayerSet::Present),
                    player::tick_serum_effect.in_set(PlayerSet::Present),
                    player::tick_pain_flash.in_set(PlayerSet::Present),
                    player::update_hand_viewmodel.in_set(PlayerSet::Present),
                    combat::apply_screen_shake.in_set(PlayerSet::Present),
                    combat::tick_hit_flash.in_set(PlayerSet::Present),
                    combat::sync_hit_flash_visual.in_set(PlayerSet::Present),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    enemy::update_enemy_ai.in_set(PlayerSet::Present),
                    enemy::radio_alert_allies.in_set(PlayerSet::Present),
                    enemy::secretary_raise_alarm.in_set(PlayerSet::Present),
                    enemy::tick_floor_alarm.in_set(PlayerSet::Present),
                    enemy::apply_floor_alarm.in_set(PlayerSet::Present),
                    enemy::deploy_tech_turrets.in_set(PlayerSet::Present),
                    enemy::update_turrets.in_set(PlayerSet::Present),
                    enemy::enemy_melee_attack.in_set(PlayerSet::Present),
                    enemy::sync_enemy_death_state.in_set(PlayerSet::Present),
                    enemy::detect_boss_presence.in_set(PlayerSet::Present),
                    enemy::tick_boss_fight.in_set(PlayerSet::Present),
                    enemy::apply_flood_hazard.in_set(PlayerSet::Present),
                    enemy::detect_warden_presence.in_set(PlayerSet::Present),
                    enemy::tick_warden_fight.in_set(PlayerSet::Present),
                    enemy::enforce_warden_pause.in_set(PlayerSet::Present),
                    enemy::apply_warden_flood.in_set(PlayerSet::Present),
                    enemy::detect_scientist_presence.in_set(PlayerSet::Present),
                    enemy::tick_scientist_fight.in_set(PlayerSet::Present),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    enemy::watch_boss_defeats.in_set(PlayerSet::Present),
                    enemy::process_enemy_deaths.in_set(PlayerSet::Present),
                    enemy::collect_loot.in_set(PlayerSet::Present),
                    combat::despawn_dead_targets.in_set(PlayerSet::Present),
                    hazard::tick_timed_valves,
                    puzzle::dna_sequencer_input,
                    interact::collect_limbs,
                    // Pain/serum flashes run through CRT post (TODO-034), not UI sprites.
                    ui::sync_crt_post_fx,
                    ui::sync_pixel_hud.before(RaycasterSystems::Render),
                    ui::sync_vitals_hud,
                    ui::sync_boss_hud,
                    puzzle::sync_dna_hud,
                    interact::update_interaction_prompt,
                    interact::try_interact,
                    interact::sync_prompt_ui,
                    game::request_elevator,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
