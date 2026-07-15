use bevy::prelude::*;

use crate::game::constants::INTERACT_DISTANCE;
use crate::game::states::GameState;
use crate::puzzle::components::{InteractAttempt, PuzzleInteractable, PuzzleSolved};

/// Resource holding the current interaction prompt to display in the HUD.
#[derive(Resource, Debug, Clone, Default)]
pub struct InteractionPrompt {
    pub text: Option<String>,
}

/// Detects the nearest puzzle interactable in front of the player and
/// updates the HUD prompt. Fires `InteractAttempt` on `E` press.
pub fn puzzle_interaction(
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<crate::player::controller::Player>>,
    puzzle_query: Query<(&Transform, &PuzzleInteractable)>,
    mut prompt: ResMut<InteractionPrompt>,
    mut interact_writer: EventWriter<InteractAttempt>,
    mut solved_writer: EventWriter<PuzzleSolved>,
) {
    prompt.text = None;

    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    // Find nearest unsolved puzzle.
    let mut nearest: Option<(&PuzzleInteractable, f32)> = None;
    for (puzzle_transform, puzzle) in puzzle_query.iter() {
        if puzzle.solved {
            continue;
        }
        let dist = player_transform
            .translation
            .distance(puzzle_transform.translation);
        if dist <= INTERACT_DISTANCE && (nearest.is_none() || dist < nearest.unwrap().1) {
            nearest = Some((puzzle, dist));
        }
    }

    if let Some((puzzle, _)) = nearest {
        prompt.text = Some(puzzle.prompt.clone());

        if keys.just_pressed(KeyCode::KeyE) {
            interact_writer.send(InteractAttempt);
            // TODO: route to the specific puzzle type's solve logic.
            // For now, auto-solve as a placeholder.
            solved_writer.send(PuzzleSolved {
                level: puzzle.level,
            });
        }
    }
}

/// Handles `PuzzleSolved` events: logs and (placeholder) advances the level.
///
/// TODO: per-level, solving the puzzle should open a door / reveal an exit
/// trigger rather than immediately advancing.
pub fn on_puzzle_solved(
    mut reader: EventReader<PuzzleSolved>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    for event in reader.read() {
        info!("Puzzle solved on level {}", event.level);
        // Placeholder: advance to next level on solve.
        let current = state.get();
        if current.level_number() == Some(event.level) {
            next_state.set(current.next_level());
        }
    }
}

/// Clears the prompt when not in a level.
pub fn clear_prompt(mut prompt: ResMut<InteractionPrompt>) {
    prompt.text = None;
}
