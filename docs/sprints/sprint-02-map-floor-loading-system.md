# Sprint 2: Map & Floor Loading System

**Owner:** Engine + Content Teams  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** Sprint 1 (raycaster + render target + state machine must exist)

---

## Goal

Build the data-driven floor loading system: RON-defined floor layouts, a loader
that spawns entities per floor cluster, the puzzle condition registry, raycast
interaction, and auto-save on elevator transitions. This sprint replaces the
current hardcoded 7-level `src/level/definitions.rs` + mesh-based `loader.rs`
with RON-driven 10-floor content.

---

## Tasks

- [ ] **[TODO-006]** Define floor data structures in RON files (`assets/floors/floor_01.ron` etc.). Include layout, palette (red → green → teal → black), ambient audio cues. (5 days)
  - **Branch:** `todo-006`
  - **Assignee:** Dev (content authoring)
  - **Dependencies:** TODO-001 (workspace), TODO-003 (raycaster map grid format)
  - **Notes:** Define a `FloorDef` RON struct: grid layout (2D array of wall/empty/special texels), palette colors, ambient audio cue refs, floor cluster tag (Human/Hybrid/Surface), puzzle refs. The palette progression red → green → teal → black maps to the 3 clusters (1-3 Human red, 4-7 Hybrid green→teal, 8-10 Surface teal→black). The existing `src/level/definitions.rs` has a `LevelDefinition` struct — port its fields (name, subtitle, ambient_light) into the RON schema and extend with grid + palette. Create all 10 floor RON files (layouts can be placeholder grids initially).

- [ ] **[TODO-007]** Floor loader system using `bevy_scene` bundles + entity spawning per floor cluster (1-3 Human, 4-7 Hybrid, 8-10 Surface). Elevator transitions with visual/audio shifts. (1 week)
  - **Branch:** `todo-007`
  - **Assignee:** Dev
  - **Dependencies:** TODO-006 (RON floor defs), TODO-005 (`ElevatorTransition` state), TODO-003 (raycaster `MapGrid` resource)
  - **Notes:** The existing `src/level/loader.rs` spawns `PbrBundle` walls/floors per level — this is replaced by loading a RON `FloorDef`, populating the raycaster's `MapGrid` resource, and spawning interactable/puzzle/enemy entities from the floor data. Floor clusters (1-3 Human, 4-7 Hybrid, 8-10 Surface) determine faction spawns and palette. Elevator transitions: fade out → save (TODO-010) → despawn current floor → load next floor → fade in with palette/audio shift. The `LevelEntity` marker component pattern from the current loader is reusable for despawn tracking.

- [ ] **[TODO-008]** Implement global `PuzzleRegistry` resource + basic condition evaluator (e.g. `has_keycard && power_restored`). (4 days)
  - **Branch:** `todo-008`
  - **Assignee:** Dev
  - **Dependencies:** TODO-005 (state machine)
  - **Notes:** A `PuzzleRegistry` resource holds named boolean flags (`has_keycard`, `power_restored`, etc.) set by puzzle solves. The condition evaluator checks compound expressions (`a && b`, `a || b`) to gate doors/elevators. This is the foundation for the full puzzle DSL (TODO-026) — keep the evaluator simple now, extend later. The existing `PuzzleSolved` event in `src/puzzle/components.rs` can feed into this registry.

- [ ] **[TODO-009]** Basic `Interactable` component for doors, terminals, valves with raycast interaction. (3 days)
  - **Branch:** `todo-009`
  - **Assignee:** Dev
  - **Dependencies:** TODO-003 (raycaster for raycast picks), TODO-007 (floor loader spawns interactables)
  - **Notes:** The existing `src/puzzle/components.rs` has `PuzzleInteractable` with distance-based interaction. Replace with a raycast-based `Interactable` component (doors, terminals, valves) that the raycaster can hit-test against. The current `puzzle_interaction` system in `src/puzzle/systems.rs` uses `Transform` distance — switch to a ray from the player's view direction hitting the interactable's billboard/cell. Keep the `InteractAttempt` event and `InteractionPrompt` resource pattern.

- [ ] **[TODO-010]** Auto-save system on elevator rides (RON serialization, 10 slots). (3 days)
  - **Branch:** `todo-010`
  - **Assignee:** Dev
  - **Dependencies:** TODO-007 (elevator transition), TODO-008 (puzzle registry state to save)
  - **Notes:** Serialize game state (current floor, inventory, puzzle flags, health, moral choices) to RON files in `saves/`. 10 slots. Triggered during `ElevatorTransition`. The `PuzzleRegistry` (TODO-008), player inventory (Sprint 3), and moral choice flags (Sprint 7) all need to be serializable — use `serde` + `bevy_reflect` where needed.

---

## Parallelization

```
TODO-006 ──> TODO-007 ──┬──> TODO-010
                        └──> TODO-009 (needs raycaster too)
TODO-005 ──> TODO-008 ──> TODO-010
```

- TODO-006 and TODO-008 can start in parallel (floor data vs puzzle registry — both depend only on Sprint 1).
- TODO-007 depends on TODO-006 (needs RON floor defs).
- TODO-009 depends on both TODO-003 (raycaster) and TODO-007 (spawned interactables).
- TODO-010 depends on TODO-007 and TODO-008 (needs elevator transition + state to save).

---

## Acceptance Criteria

- [ ] 10 RON floor files exist in `assets/floors/` with layout, palette, and audio cues.
- [ ] Loading a floor populates the raycaster `MapGrid` and spawns entities.
- [ ] Floor clusters (Human/Hybrid/Surface) apply correct palette and faction spawns.
- [ ] Elevator transition: save → unload → load → palette/audio shift.
- [ ] `PuzzleRegistry` evaluates compound conditions and gates doors.
- [ ] Raycast interaction works on doors, terminals, valves with HUD prompts.
- [ ] Auto-save writes/loads RON save slots during elevator transitions.
