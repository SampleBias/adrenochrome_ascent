//! Tracker-style music, footsteps, and PA lines (TODO-032).

use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::prelude::*;

use crate::enemy::{BossFight, FloorAlarm, ScientistFight, WardenOverrides};
use crate::floor_loader::LoadedFloorInfo;
use crate::game::{CurrentFloor, GameSettings, GameState};
use crate::player::{Player, PlayerMotor};
use crate::puzzle::DnaSequencer;

/// Handles for all baked lo-fi SFX / beds.
#[derive(Resource, Clone)]
pub struct AudioBank {
    pub music_human: Handle<AudioSource>,
    pub music_hybrid: Handle<AudioSource>,
    pub music_surface: Handle<AudioSource>,
    pub music_combat: Handle<AudioSource>,
    pub foot_wet: Handle<AudioSource>,
    pub foot_industrial: Handle<AudioSource>,
    pub foot_clean: Handle<AudioSource>,
    pub pa_line: Handle<AudioSource>,
    pub ambient_hum: Handle<AudioSource>,
}

impl AudioBank {
    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            music_human: asset_server.load("audio/music_human.wav"),
            music_hybrid: asset_server.load("audio/music_hybrid.wav"),
            music_surface: asset_server.load("audio/music_surface.wav"),
            music_combat: asset_server.load("audio/music_combat.wav"),
            foot_wet: asset_server.load("audio/foot_wet.wav"),
            foot_industrial: asset_server.load("audio/foot_industrial.wav"),
            foot_clean: asset_server.load("audio/foot_clean.wav"),
            pa_line: asset_server.load("audio/pa_line.wav"),
            ambient_hum: asset_server.load("audio/ambient_hum.wav"),
        }
    }

    pub fn music_for_floor(&self, floor: u8) -> Handle<AudioSource> {
        match floor {
            1..=3 => self.music_human.clone(),
            4..=7 => self.music_hybrid.clone(),
            _ => self.music_surface.clone(),
        }
    }

    pub fn foot_for_floor(&self, floor: u8) -> Handle<AudioSource> {
        match floor {
            1..=3 => self.foot_wet.clone(),
            4..=7 => self.foot_industrial.clone(),
            _ => self.foot_clean.clone(),
        }
    }
}

#[derive(Component)]
pub(crate) struct MusicBed;

#[derive(Component)]
pub(crate) struct CombatBed;

#[derive(Component)]
pub(crate) struct AmbientBed;

#[derive(Component)]
struct OneShotSfx;

/// Footstep stride accumulator.
#[derive(Resource, Debug, Clone, Copy)]
pub struct FootstepState {
    pub distance: f32,
    pub stride: f32,
}

impl Default for FootstepState {
    fn default() -> Self {
        Self {
            distance: 0.0,
            stride: 1.15,
        }
    }
}

/// PA banner + one-shot voice trigger (TODO-032).
#[derive(Resource, Debug, Clone, Default)]
pub struct PaAnnouncement {
    pub text: String,
    pub time_left: f32,
    pub pending_voice: bool,
}

impl PaAnnouncement {
    pub fn announce(&mut self, text: impl Into<String>, duration: f32) {
        self.text = text.into();
        self.time_left = duration;
        self.pending_voice = true;
    }
}

pub fn load_audio_bank(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioBank::load(&asset_server));
}

/// Start cluster music + ambient when entering a floor.
pub(crate) fn start_floor_audio(
    mut commands: Commands,
    bank: Res<AudioBank>,
    settings: Res<GameSettings>,
    floor: Res<CurrentFloor>,
    floor_info: Res<LoadedFloorInfo>,
    mut pa: ResMut<PaAnnouncement>,
    music: Query<Entity, With<MusicBed>>,
    ambient: Query<Entity, With<AmbientBed>>,
    combat: Query<Entity, With<CombatBed>>,
) {
    for e in music.iter().chain(ambient.iter()).chain(combat.iter()) {
        commands.entity(e).despawn();
    }

    let music_vol = settings.music_volume.clamp(0.0, 1.0);
    let music_handle = bank.music_for_floor(floor.number);
    commands.spawn((
        Name::new("MusicBed"),
        MusicBed,
        AudioPlayer::new(music_handle),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(music_vol)),
    ));
    commands.spawn((
        Name::new("AmbientBed"),
        AmbientBed,
        AudioPlayer::new(bank.ambient_hum.clone()),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(music_vol * 0.45)),
    ));

    let line = if floor_info.intro_text.is_empty() {
        format!("FLOOR {} — SYSTEM ONLINE", floor.number)
    } else {
        floor_info.intro_text.chars().take(42).collect()
    };
    pa.announce(line, 4.5);
    bevy::log::info!("Floor audio: cluster bed + PA");
}

fn stop_floor_audio(
    mut commands: Commands,
    music: Query<Entity, Or<(With<MusicBed>, With<AmbientBed>, With<CombatBed>)>>,
    mut pa: ResMut<PaAnnouncement>,
) {
    for e in &music {
        commands.entity(e).despawn();
    }
    pa.text.clear();
    pa.time_left = 0.0;
}

/// Intensify with a combat bed while bosses / alarms are hot.
fn sync_combat_music(
    mut commands: Commands,
    bank: Res<AudioBank>,
    settings: Res<GameSettings>,
    fight: Res<BossFight>,
    warden: Res<WardenOverrides>,
    scientist: Res<ScientistFight>,
    alarm: Res<FloorAlarm>,
    combat: Query<Entity, With<CombatBed>>,
) {
    let hot = fight.active
        || warden.active
        || scientist.active
        || alarm.active;
    let playing = !combat.is_empty();
    if hot && !playing {
        let vol = (settings.music_volume * 1.25).clamp(0.0, 1.0);
        commands.spawn((
            Name::new("CombatBed"),
            CombatBed,
            AudioPlayer::new(bank.music_combat.clone()),
            PlaybackSettings::LOOP.with_volume(Volume::Linear(vol)),
        ));
    } else if !hot && playing {
        for e in &combat {
            commands.entity(e).despawn();
        }
    }
}

fn play_footsteps(
    mut commands: Commands,
    time: Res<Time>,
    bank: Res<AudioBank>,
    settings: Res<GameSettings>,
    floor: Res<CurrentFloor>,
    mut steps: ResMut<FootstepState>,
    player: Query<&PlayerMotor, With<Player>>,
) {
    let Ok(motor) = player.single() else {
        return;
    };
    let speed = motor.velocity.length();
    if speed < 0.35 {
        steps.distance = 0.0;
        return;
    }
    steps.distance += speed * time.delta_secs();
    let stride = if motor.is_sprinting {
        steps.stride * 0.75
    } else {
        steps.stride
    };
    if steps.distance < stride {
        return;
    }
    steps.distance = 0.0;
    let vol = (settings.sfx_volume * 0.82).clamp(0.0, 1.0);
    commands.spawn((
        Name::new("Footstep"),
        OneShotSfx,
        AudioPlayer::new(bank.foot_for_floor(floor.number)),
        PlaybackSettings::DESPAWN.with_volume(Volume::Linear(vol)),
    ));
}

fn tick_pa_announcements(
    mut commands: Commands,
    time: Res<Time>,
    bank: Res<AudioBank>,
    settings: Res<GameSettings>,
    mut pa: ResMut<PaAnnouncement>,
    dna: Res<DnaSequencer>,
    alarm: Res<FloorAlarm>,
    mut last_alarm: Local<bool>,
) {
    if pa.pending_voice {
        pa.pending_voice = false;
        let vol = settings.sfx_volume.clamp(0.0, 1.0);
        commands.spawn((
            Name::new("PaVoice"),
            OneShotSfx,
            AudioPlayer::new(bank.pa_line.clone()),
            PlaybackSettings::DESPAWN.with_volume(Volume::Linear(vol)),
        ));
    }
    if pa.time_left > 0.0 {
        pa.time_left = (pa.time_left - time.delta_secs()).max(0.0);
        if pa.time_left <= 0.0 {
            pa.text.clear();
        }
    }

    // Alarm PA sting
    if alarm.active && !*last_alarm {
        pa.announce("SECURITY ALERT — ALL UNITS", 3.5);
    }
    *last_alarm = alarm.active;

    // DNA session cue (soft)
    if dna.active && pa.time_left <= 0.0 {
        // don't spam — only when banner empty; optional
    }
}

/// Boss phase PA teases.
fn pa_boss_phase_hooks(
    fight: Res<BossFight>,
    warden: Res<WardenOverrides>,
    scientist: Res<ScientistFight>,
    mut pa: ResMut<PaAnnouncement>,
    mut last_lt: Local<u8>,
    mut last_w: Local<u8>,
    mut last_s: Local<u8>,
) {
    if fight.active && fight.phase != *last_lt {
        *last_lt = fight.phase;
        pa.announce(format!("LIEUTENANT — PHASE {}", fight.phase), 3.0);
    }
    if warden.active && warden.phase != *last_w {
        *last_w = warden.phase;
        pa.announce(format!("WARDEN PROTOCOL {}", warden.phase), 3.0);
    }
    if scientist.active && scientist.phase != *last_s {
        *last_s = scientist.phase;
        pa.announce(format!("SEQUENCE UNSTABLE — P{}", scientist.phase), 3.0);
    }
}

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FootstepState>()
            .init_resource::<PaAnnouncement>()
            .add_systems(Startup, load_audio_bank)
            .add_systems(OnExit(GameState::InGame), stop_floor_audio)
            .add_systems(
                Update,
                (
                    play_footsteps,
                    sync_combat_music,
                    tick_pa_announcements,
                    pa_boss_phase_hooks,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
