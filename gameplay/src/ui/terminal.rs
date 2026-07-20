//! Full-resolution egui terminal / DNA panels (TODO-033).

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext};

use adrenochrome_engine::UpscaleCamera;

use crate::combat::CombatTarget;
use crate::enemy::{Enemy, FloorAlarm, ScientistFight};
use crate::game::GameState;
use crate::puzzle::{resolve_dna_if_complete, DnaSequencer, PuzzleRegistry};
use adrenochrome_engine::MapGrid;

/// Generic terminal session opened from floor interactables.
#[derive(Resource, Debug, Clone, Default)]
pub struct TerminalSession {
    pub active: bool,
    pub title: String,
    pub body: String,
    pub set_flag: Option<String>,
    pub confirmed: bool,
}

impl TerminalSession {
    pub fn open(
        &mut self,
        title: impl Into<String>,
        body: impl Into<String>,
        set_flag: Option<String>,
    ) {
        self.active = true;
        self.title = title.into();
        self.body = body.into();
        self.set_flag = set_flag;
        self.confirmed = false;
    }

    pub fn close(&mut self) {
        self.active = false;
        self.title.clear();
        self.body.clear();
        self.set_flag = None;
        self.confirmed = false;
    }
}

pub fn attach_egui_primary_context(
    mut commands: Commands,
    cams: Query<Entity, (With<UpscaleCamera>, Without<PrimaryEguiContext>)>,
) {
    for entity in &cams {
        commands.entity(entity).insert(PrimaryEguiContext);
    }
}

/// egui DNA sequencer + terminal panels at window resolution.
pub fn draw_egui_panels(
    mut contexts: EguiContexts,
    mut dna: ResMut<DnaSequencer>,
    mut terminal: ResMut<TerminalSession>,
    mut registry: ResMut<PuzzleRegistry>,
    mut map: ResMut<MapGrid>,
    mut fight: ResMut<ScientistFight>,
    mut alarm: ResMut<FloorAlarm>,
    mut bosses: Query<(&Enemy, &mut CombatTarget)>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    if dna.active {
        egui::Window::new("DNA SEQUENCER")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Enter base sequence (A C G T). Esc cancels.");
                ui.separator();
                let target: String = dna.target.iter().collect();
                let input: String = dna.input.iter().collect();
                ui.monospace(format!("TARGET  {target}"));
                ui.monospace(format!("INPUT   {input}_"));
                ui.separator();
                ui.horizontal(|ui| {
                    for (label, base) in [("A", 'A'), ("C", 'C'), ("G", 'G'), ("T", 'T')] {
                        if ui.button(label).clicked() {
                            let _ = dna.push(base);
                        }
                    }
                    if ui.button("⌫").clicked() {
                        dna.input.pop();
                    }
                    if ui.button("Cancel").clicked() {
                        dna.active = false;
                        dna.input.clear();
                    }
                });
            });
        resolve_dna_if_complete(
            &mut dna,
            &mut registry,
            &mut map,
            &mut fight,
            &mut alarm,
            &mut bosses,
        );
    }

    if terminal.active {
        let title = terminal.title.clone();
        egui::Window::new(title)
            .collapsible(false)
            .resizable(true)
            .default_width(420.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, -40.0])
            .show(ctx, |ui| {
                ui.label(&terminal.body);
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Confirm").clicked() {
                        terminal.confirmed = true;
                    }
                    if ui.button("Close").clicked() {
                        terminal.close();
                    }
                });
            });
        if terminal.confirmed {
            if let Some(flag) = terminal.set_flag.clone() {
                registry.set(flag, true);
            }
            terminal.close();
        }
    }

    Ok(())
}

pub struct TerminalUiPlugin;

impl Plugin for TerminalUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerminalSession>()
            .add_plugins(EguiPlugin::default())
            .add_systems(
                Startup,
                attach_egui_primary_context.after(adrenochrome_engine::setup_render_target),
            )
            .add_systems(
                EguiPrimaryContextPass,
                draw_egui_panels.run_if(in_state(GameState::InGame)),
            );
    }
}
