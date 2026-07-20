//! Puzzle flag registry, DSL effects, DNA sequencer (TODO-008 / TODO-026).

mod dna;
mod effects;
mod registry;

pub use dna::{
    dna_sequencer_input, resolve_dna_if_complete, sync_dna_hud, DnaHudText, DnaSequencer,
};
pub use effects::apply_effects;
pub use registry::PuzzleRegistry;
