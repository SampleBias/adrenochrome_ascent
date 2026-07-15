use bevy::prelude::*;

use crate::game::conditions::{in_any_level, in_menu};
use crate::puzzle::components::{InteractAttempt, PuzzleSolved};
use crate::puzzle::systems::{
    clear_prompt, on_puzzle_solved, puzzle_interaction, InteractionPrompt,
};

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractAttempt>()
            .add_event::<PuzzleSolved>()
            .init_resource::<InteractionPrompt>()
            .add_systems(
                Update,
                (puzzle_interaction, on_puzzle_solved).run_if(in_any_level),
            )
            .add_systems(Update, clear_prompt.run_if(in_menu));
    }
}
