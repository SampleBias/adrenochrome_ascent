//! RON floor definitions (TODO-006).
//!
//! Files live in `assets/floors/floor_XX.ron` and are loaded by the floor
//! loader (TODO-007) into `MapGrid` + spawned entities.

use serde::{Deserialize, Serialize};

use adrenochrome_engine::{MapGrid, Palette};

/// Which narrative/faction cluster a floor belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FloorCluster {
    Human,
    Hybrid,
    Surface,
}

impl FloorCluster {
    pub fn from_floor_number(n: u8) -> Self {
        match n {
            1..=3 => Self::Human,
            4..=7 => Self::Hybrid,
            _ => Self::Surface,
        }
    }
}

/// Serializable palette name matching [`Palette`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaletteId {
    Red,
    Green,
    Teal,
    Black,
}

impl From<PaletteId> for Palette {
    fn from(value: PaletteId) -> Self {
        match value {
            PaletteId::Red => Palette::Red,
            PaletteId::Green => Palette::Green,
            PaletteId::Teal => Palette::Teal,
            PaletteId::Black => Palette::Black,
        }
    }
}

impl PaletteId {
    pub fn for_floor(n: u8) -> Self {
        match n {
            1..=3 => Self::Red,
            4..=7 => Self::Green,
            8..=9 => Self::Teal,
            _ => Self::Black,
        }
    }
}

/// Faction tier (maps to boss hierarchy).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionId {
    Mob,
    Security,
    Research,
    Executive,
}

/// Spawnable enemy archetypes (Mob + Security + Research tiers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyArchetypeId {
    // Mob (Sprint 4)
    Thug,
    Heavy,
    Zed,
    Lieutenant,
    // Security (Sprint 5)
    RiotGuard,
    PatrolSecurity,
    HazardTech,
    Warden,
    // Research (Sprint 6)
    ResearcherMale,
    ResearcherFemale,
    MutatedAide,
    SerumZombie,
    Scientist,
    // Executive (Sprint 7)
    Bodyguard,
    AdminSecretary,
    LimoDriver,
    Ceo,
}

/// What happens when the player uses an interactable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractAction {
    /// Set a puzzle flag true.
    SetFlag(String),
    /// Clear a solid map cell (open a door) and optionally set a flag.
    OpenDoor {
        cell: (i32, i32),
        flag: Option<String>,
    },
    /// Call the elevator (gated by optional condition).
    CallElevator,
    /// Flip the moral-choice ending toward Released.
    ReleaseSubjects,
    /// Optional side-puzzle moral choice (TODO-031): sets flag + increments moral score.
    MoralChoice {
        flag: String,
        /// If true, also sets `subjects_released` / EndingKind::Released.
        #[serde(default)]
        release_subjects: bool,
        /// Points added to `moral_score` counter.
        #[serde(default = "default_one")]
        score: i32,
    },
    /// Trigger a floor-wide stealth alarm (TODO-028).
    TriggerAlarm,
    /// Start / refresh a timed valve window (TODO-022).
    TimedValve {
        flag: String,
        window_secs: f32,
    },
    /// Collect a biometric limb (TODO-026).
    CollectLimb {
        #[serde(default = "default_one")]
        amount: i32,
    },
    /// Activate a named puzzle (DNA sequencer, etc.) — TODO-026.
    StartPuzzle(String),
    /// Open a full-res egui terminal panel (TODO-033).
    OpenTerminal {
        title: String,
        body: String,
        /// Flag set when the player confirms / closes the terminal.
        #[serde(default)]
        set_flag: Option<String>,
    },
    /// Fire a PA banner + voice sting (TODO-036).
    AnnouncePa {
        text: String,
        #[serde(default = "default_pa_duration")]
        duration: f32,
    },
    /// Apply puzzle DSL effects if condition holds (TODO-026).
    RunEffects {
        require: Option<String>,
        effects: Vec<PuzzleEffectId>,
    },
}

fn default_pa_duration() -> f32 {
    4.0
}

fn default_one() -> i32 {
    1
}

/// Serializable puzzle effects for RON floor / puzzle defs (TODO-026).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PuzzleEffectId {
    SetFlag(String),
    ClearFlag(String),
    OpenDoor {
        cell: (i32, i32),
        flag: Option<String>,
    },
    AddCounter {
        name: String,
        amount: i32,
    },
    SetCounter {
        name: String,
        value: i32,
    },
    DamageBoss(f32),
    AdvanceBossPhase,
    /// Increment moral score / set a moral flag (TODO-031).
    MoralBump {
        flag: String,
        #[serde(default = "default_one")]
        score: i32,
    },
    TriggerAlarm,
}

/// Kind of world object to spawn from floor data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityKind {
    /// Decorative / item billboard.
    Billboard {
        texture_id: usize,
        scale: f32,
    },
    /// Combat enemy with AI (TODO-015+).
    Enemy {
        faction: FactionId,
        archetype: EnemyArchetypeId,
        scale: f32,
        /// Optional patrol path in map-cell units.
        #[serde(default)]
        waypoints: Vec<(f32, f32)>,
        /// Initial facing yaw (radians); 0 = +X.
        #[serde(default)]
        yaw: f32,
    },
    /// Grid-pushable crate (TODO-022). Occupies the floor cell under `pos`.
    Crate {
        #[serde(default = "default_crate_scale")]
        scale: f32,
    },
    /// Stationary security turret (TODO-020).
    Turret {
        #[serde(default = "default_turret_yaw")]
        yaw: f32,
        #[serde(default = "default_turret_scale")]
        scale: f32,
    },
    /// Biometric limb pickup (TODO-026).
    Limb {
        #[serde(default = "default_one")]
        amount: i32,
        #[serde(default = "default_limb_scale")]
        scale: f32,
    },
    /// Raycast-interactable (door, terminal, valve, elevator).
    Interactable {
        prompt: String,
        /// Optional condition expression (`has_keycard && power_restored`).
        require: Option<String>,
        action: InteractAction,
        /// Billboard sprite shown in-world (None = invisible hotspot).
        texture_id: Option<usize>,
        scale: f32,
    },
}

fn default_crate_scale() -> f32 {
    0.7
}
fn default_turret_yaw() -> f32 {
    0.0
}
fn default_turret_scale() -> f32 {
    0.55
}
fn default_limb_scale() -> f32 {
    0.4
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityDef {
    pub pos: (f32, f32),
    pub kind: EntityKind,
}

/// Boss / alarm wave tuning authored per floor (TODO-036).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WaveTuning {
    /// Max living grunts before summons pause.
    #[serde(default = "default_max_grunts")]
    pub max_grunts: u8,
    /// Seconds between summons.
    #[serde(default = "default_wave_cooldown")]
    pub cooldown_secs: f32,
}

fn default_max_grunts() -> u8 {
    3
}
fn default_wave_cooldown() -> f32 {
    8.0
}

impl Default for WaveTuning {
    fn default() -> Self {
        Self {
            max_grunts: default_max_grunts(),
            cooldown_secs: default_wave_cooldown(),
        }
    }
}

impl WaveTuning {
    pub fn for_cluster(cluster: FloorCluster) -> Self {
        match cluster {
            FloorCluster::Human => Self {
                max_grunts: 2,
                cooldown_secs: 9.5,
            },
            FloorCluster::Hybrid => Self {
                max_grunts: 3,
                cooldown_secs: 8.0,
            },
            FloorCluster::Surface => Self {
                max_grunts: 4,
                cooldown_secs: 6.5,
            },
        }
    }
}

/// One floor of the ascent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloorDef {
    pub number: u8,
    pub name: String,
    pub subtitle: String,
    pub cluster: FloorCluster,
    pub palette: PaletteId,
    /// Ambient audio cue asset key (playback is TODO-032).
    pub ambient_audio: String,
    pub intro_text: String,
    pub player_spawn: (f32, f32),
    /// Radians; 0 = +X.
    pub player_yaw: f32,
    /// Row-major ASCII grid (same charset as [`MapGrid::from_rows`]).
    pub rows: Vec<String>,
    pub entities: Vec<EntityDef>,
    /// Optional boss-wave tuning; defaults from cluster if omitted.
    #[serde(default)]
    pub wave_tuning: Option<WaveTuning>,
}

impl FloorDef {
    pub fn resolved_wave_tuning(&self) -> WaveTuning {
        self.wave_tuning
            .unwrap_or_else(|| WaveTuning::for_cluster(self.cluster))
    }

    pub fn to_map_grid(&self) -> MapGrid {
        let refs: Vec<&str> = self.rows.iter().map(|s| s.as_str()).collect();
        MapGrid::from_rows(&refs)
    }

    pub fn asset_path(number: u8) -> String {
        format!("floors/floor_{number:02}.ron")
    }
}

fn floor_asset_candidates(rel: &str) -> Vec<std::path::PathBuf> {
    let mut candidates = vec![
        std::path::PathBuf::from("assets").join(rel),
        std::path::PathBuf::from("adrenochrome_ascent/assets").join(rel),
    ];
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        // `content/` or `gameplay/` crate → workspace `assets/`
        candidates.push(std::path::PathBuf::from(&manifest).join("../assets").join(rel));
        candidates.push(std::path::PathBuf::from(&manifest).join("../../assets").join(rel));
    }
    candidates
}

/// Load a floor definition from an absolute or cwd-relative filesystem path.
pub fn load_floor_file(path: &std::path::Path) -> Result<FloorDef, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| format!("read {}: {e}", path.display()))?;
    ron::from_str::<FloorDef>(&text).map_err(|e| format!("parse {}: {e}", path.display()))
}

/// Resolve a path under `assets/` (e.g. `floors/ending_road.ron`).
pub fn load_floor_asset(rel: &str) -> Result<FloorDef, String> {
    for path in floor_asset_candidates(rel) {
        if path.exists() {
            return load_floor_file(&path);
        }
    }
    Err(format!("floor asset not found (tried assets/{rel})"))
}

/// Resolve `assets/floors/floor_XX.ron` from cwd or workspace root.
pub fn load_floor_number(number: u8) -> Result<FloorDef, String> {
    let rel = FloorDef::asset_path(number);
    load_floor_asset(&rel).map_err(|_| format!("floor {number} not found (tried assets/{rel})"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_floor_ron() {
        let src = concat!(
            "FloorDef(",
            "number: 1,",
            "name: \"Test\",",
            "subtitle: \"Sub\",",
            "cluster: Human,",
            "palette: Red,",
            "ambient_audio: \"ambient/test\",",
            "intro_text: \"Hi\",",
            "player_spawn: (1.5, 1.5),",
            "player_yaw: 0.0,",
            "rows: [\"###\", \"#.#\", \"###\"],",
            "entities: [],",
            ")",
        );
        let def: FloorDef = ron::from_str(src).unwrap();
        assert_eq!(def.number, 1);
        let map = def.to_map_grid();
        assert!(!map.is_solid(1, 1));
    }

    #[test]
    fn loads_all_ten_authored_floors() {
        for n in 1..=10u8 {
            let def = load_floor_number(n).unwrap_or_else(|e| panic!("floor {n}: {e}"));
            assert_eq!(def.number, n);
            assert!(!def.rows.is_empty());
            let _ = def.to_map_grid();
        }
    }

    #[test]
    fn loads_ending_road_cinematic() {
        let def = load_floor_asset("floors/ending_road.ron")
            .unwrap_or_else(|e| panic!("ending_road: {e}"));
        assert_eq!(def.number, 99);
        assert_eq!(def.cluster, FloorCluster::Surface);
        assert!(!def.entities.is_empty());
    }

    #[test]
    fn parses_enemy_entity_kind() {
        let src = concat!(
            "FloorDef(",
            "number: 1, name: \"T\", subtitle: \"S\", cluster: Human, palette: Red,",
            "ambient_audio: \"a\", intro_text: \"i\", player_spawn: (1.5, 1.5), player_yaw: 0.0,",
            "rows: [\"###\", \"#.#\", \"###\"],",
            "entities: [EntityDef(pos: (1.5, 1.5), kind: Enemy(",
            "faction: Mob, archetype: Thug, scale: 1.0,",
            "waypoints: [(1.5, 1.5), (1.5, 1.8)], yaw: 0.0))],",
            ")",
        );
        let def: FloorDef = ron::from_str(src).unwrap();
        match &def.entities[0].kind {
            EntityKind::Enemy {
                faction,
                archetype,
                waypoints,
                ..
            } => {
                assert_eq!(*faction, FactionId::Mob);
                assert_eq!(*archetype, EnemyArchetypeId::Thug);
                assert_eq!(waypoints.len(), 2);
            }
            _ => panic!("expected Enemy"),
        }
    }
}
