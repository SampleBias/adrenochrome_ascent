//! DNA sequencer mini-game (TODO-025 / TODO-026).

use bevy::prelude::*;

use adrenochrome_content::PuzzleEffectId;
use adrenochrome_engine::MapGrid;

use crate::combat::CombatTarget;
use crate::enemy::{Enemy, FloorAlarm, ScientistFight};

use super::effects::apply_effects;
use super::PuzzleRegistry;

/// Active DNA sequencer session.
#[derive(Resource, Debug, Clone)]
pub struct DnaSequencer {
    pub active: bool,
    pub target: Vec<char>,
    pub input: Vec<char>,
    pub puzzle_id: String,
}

impl Default for DnaSequencer {
    fn default() -> Self {
        Self {
            active: false,
            target: vec!['A', 'C', 'G', 'T'],
            input: Vec::new(),
            puzzle_id: String::new(),
        }
    }
}

impl DnaSequencer {
    pub fn start(&mut self, id: impl Into<String>, target: &str) {
        self.active = true;
        self.puzzle_id = id.into();
        self.target = target.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        self.input.clear();
        info!(
            "DNA sequencer '{}': enter {:?}",
            self.puzzle_id, self.target
        );
    }

    pub fn push(&mut self, base: char) -> bool {
        if !self.active {
            return false;
        }
        let b = base.to_ascii_uppercase();
        if !matches!(b, 'A' | 'C' | 'G' | 'T') {
            return false;
        }
        self.input.push(b);
        true
    }

    pub fn matches_target(&self) -> bool {
        self.input == self.target
    }

    pub fn is_complete(&self) -> bool {
        self.input.len() >= self.target.len()
    }
}

/// Keyboard input while sequencer is active (A/C/G/T or 1–4).
pub fn dna_sequencer_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut dna: ResMut<DnaSequencer>,
    mut registry: ResMut<PuzzleRegistry>,
    mut map: ResMut<MapGrid>,
    mut fight: ResMut<ScientistFight>,
    mut alarm: ResMut<FloorAlarm>,
    mut bosses: Query<(&Enemy, &mut CombatTarget)>,
) {
    if !dna.active {
        return;
    }
    let base = if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::Digit1) {
        Some('A')
    } else if keys.just_pressed(KeyCode::KeyC) || keys.just_pressed(KeyCode::Digit2) {
        Some('C')
    } else if keys.just_pressed(KeyCode::KeyG) || keys.just_pressed(KeyCode::Digit3) {
        Some('G')
    } else if keys.just_pressed(KeyCode::KeyT) || keys.just_pressed(KeyCode::Digit4) {
        Some('T')
    } else if keys.just_pressed(KeyCode::Backspace) {
        dna.input.pop();
        return;
    } else if keys.just_pressed(KeyCode::Escape) {
        dna.active = false;
        dna.input.clear();
        return;
    } else {
        None
    };

    let Some(b) = base else {
        return;
    };
    if !dna.push(b) {
        return;
    }
    resolve_dna_if_complete(
        &mut dna,
        &mut registry,
        &mut map,
        &mut fight,
        &mut alarm,
        &mut bosses,
    );
}

/// Shared completion handler for keyboard + egui (TODO-033).
pub fn resolve_dna_if_complete(
    dna: &mut DnaSequencer,
    registry: &mut PuzzleRegistry,
    map: &mut MapGrid,
    fight: &mut ScientistFight,
    alarm: &mut FloorAlarm,
    bosses: &mut Query<(&Enemy, &mut CombatTarget)>,
) {
    if !dna.active || !dna.is_complete() {
        return;
    }
    if dna.matches_target() {
        registry.set("dna_sequence_correct", true);
        let effects = vec![
            PuzzleEffectId::SetFlag("dna_sequence_correct".into()),
            PuzzleEffectId::DamageBoss(55.0),
            PuzzleEffectId::AdvanceBossPhase,
        ];
        apply_effects(effects.as_slice(), registry, map, fight, alarm, bosses);
        info!("DNA sequence correct — boss damaged");
    } else {
        registry.set("dna_sequence_correct", false);
        info!("DNA sequence wrong: {:?} vs {:?}", dna.input, dna.target);
    }
    dna.active = false;
    dna.input.clear();
}

#[derive(Component)]
pub struct DnaHudText;

pub fn sync_dna_hud(dna: Res<DnaSequencer>, mut texts: Query<&mut Text, With<DnaHudText>>) {
    let Ok(mut text) = texts.single_mut() else {
        return;
    };
    if !dna.active {
        **text = String::new();
        return;
    }
    let target: String = dna.target.iter().collect();
    let input: String = dna.input.iter().collect();
    **text = format!("DNA [{target}]  entered: {input}_   (A/C/G/T or 1-4, Esc cancel)");
}
