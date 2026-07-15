# Sprint 1: Project Bootstrap & Core Engine Foundation

**Owner:** Engine Team (Master Devs lead)  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** None (this is the foundation sprint)

---

## Goal

Migrate the existing Bevy 0.14 3D scaffold to a Bevy 0.15+ raycaster-based engine
with a 320×200 CRT aesthetic. Establish the Cargo workspace structure, core
rendering pipeline, first-person controller, and game state machine that all
subsequent sprints build on.

> ⚠️ **Migration sprint.** The current codebase is a Bevy 0.14 3D first-person
> scaffold (`PbrBundle`, `Camera3d`, mesh-based walls). This sprint replaces the
> renderer with a software raycaster and restructures the project into Cargo
> workspaces. Much of the existing `src/level/loader.rs` and `src/player/controller.rs`
> will be rewritten, not extended.

---

## Tasks

- [x] **[TODO-001]** Initialize Bevy 0.15+ project with Cargo workspaces (engine, gameplay, content crates). Add core plugins: `bevy_ecs`, `bevy_sprite`, `bevy_asset`, `bevy_audio`. (2 days)
  - **Branch:** `todo-001`
  - **Assignee:** Master Dev
  - **Notes:** Current `Cargo.toml` is a single crate on Bevy 0.14. Split into workspace members: `engine/` (raycaster, rendering), `gameplay/` (ECS systems, player, enemy, puzzle), `content/` (RON floor data, asset definitions). Bump `bevy = "0.15"`.
  - **Parallelizable with:** TODO-005 (state enum is independent of workspace structure)
  - **Completed:** Bevy bumped to 0.19.0. Workspace created with 4 members: `engine/`, `gameplay/`, `content/`, `app/` (binary). Old Bevy 0.14 3D code backed up to `legacy/src_014/`. `cargo build` succeeds.

- [ ] **[TODO-002]** Set up 320×200 internal render target with nearest-neighbor upscale + basic CRT shader pipeline supporting palette swaps. (4 days)
  - **Branch:** `todo-002`
  - **Assignee:** Dev (strong Rust / rendering)
  - **Dependencies:** TODO-001 (workspace must exist)
  - **Notes:** Render the game to a 320×200 `Image` target, then upscale with nearest-neighbor to the window resolution via a fullscreen quad + custom `Material` shader. Add a palette-swap uniform so floors can shift colors (red → green → teal → black per TODO-006). No CRT curvature/scanlines yet — that's TODO-034.

- [ ] **[MASTER]** **[TODO-003]** Implement custom software raycaster (Doom-style) or `bevy_voxel` hybrid. Support billboard sprites for enemies and hand. (1 week)
  - **Branch:** `todo-003`
  - **Assignee:** Master Dev
  - **Dependencies:** TODO-001, TODO-002 (render target must exist)
  - **Notes:** This is the core engine piece. DDA raycasting against a 2D grid map, textured walls, billboard sprites for enemies/items/hand. The current `src/level/loader.rs` spawns 3D `PbrBundle` walls — this replaces that entirely with a grid-based map the raycaster reads. Consider a `MapGrid` resource (2D array of wall texels) that the floor loader (TODO-007) populates.

- [ ] **[TODO-004]** Basic first-person controller (WASD + mouse look, Doom-style movement/friction). (3 days)
  - **Branch:** `todo-004`
  - **Assignee:** Dev
  - **Dependencies:** TODO-003 (raycaster defines collision against the map grid)
  - **Notes:** The existing `src/player/controller.rs` has a kinematic 3D controller with yaw/pitch and crouch. Adapt to 2D-plane movement (x/z in world = x/y on the map grid) with Doom-style acceleration/friction. Collision is grid-cell based (wall cells block movement), not physics-engine-based. Crouch may be dropped or repurposed. Mouse look only affects yaw for the raycaster; pitch is used for the hand sprite / interaction reticle only.

- [ ] **[TODO-005]** Create main game state enum (`MainMenu`, `InGame`, `ElevatorTransition`, `Ending`). (2 days)
  - **Branch:** `todo-005`
  - **Assignee:** Dev
  - **Dependencies:** TODO-001 (workspace)
  - **Notes:** The existing `src/game/states.rs` has `GameState` with `Level1..7`, `Paused`, `GameOver`, `Victory`. Replace with the new flow: `MainMenu`, `InGame`, `ElevatorTransition`, `Ending`. The 10-floor progression lives inside `InGame` via a `CurrentFloor` resource (u8 1..=10), not as separate states. `ElevatorTransition` handles the load/save + visual/audio shift between floors. `Ending` branches into the moral-choice endings (TODO-029). Update `src/game/conditions.rs` and `src/game/plugin.rs` accordingly.

---

## Parallelization

```
TODO-001 ──┬──> TODO-002 ──> TODO-003 ──> TODO-004
           └──> TODO-005
```

- TODO-001 must complete first (workspace + Bevy 0.15 bump).
- TODO-005 can run in parallel with TODO-002/003 (it's a state-machine refactor, independent of rendering).
- TODO-004 depends on TODO-003 (collision model is defined by the raycaster).

---

## Acceptance Criteria

- [ ] `cargo build` succeeds on Bevy 0.15+ with workspace structure.
- [ ] A 320×200 render target is visible, upscaled to the window.
- [ ] Raycaster renders a textured wall grid from a test map.
- [ ] Player can move (WASD) and look (mouse) with grid-based collision.
- [ ] Game state cycles: `MainMenu` → `InGame` → `ElevatorTransition` → `Ending`.
- [ ] No 3D `PbrBundle`/`Camera3d` remains in the gameplay path (raycaster replaces it).
