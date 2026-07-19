# Sprint 2: Map & Floor Loading System

**Owner:** Engine + Content Teams  
**Estimate:** 2-4 weeks  
**Status:** Complete  
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

- [x] **[TODO-006]** Define floor data structures in RON files (`assets/floors/floor_01.ron` etc.). Include layout, palette (red → green → teal → black), ambient audio cues. (5 days)
  - **Branch:** `sprint-02`
  - **Completed:** `content::FloorDef` + 10 authored RON floors with cluster/palette/audio/entities.

- [x] **[TODO-007]** Floor loader system using `bevy_scene` bundles + entity spawning per floor cluster (1-3 Human, 4-7 Hybrid, 8-10 Surface). Elevator transitions with visual/audio shifts. (1 week)
  - **Completed:** `gameplay::floor_loader` populates `MapGrid` / `RayCamera` / palette, spawns `FloorEntity` billboards + interactables; elevator advances floor and reloads.

- [x] **[TODO-008]** Implement global `PuzzleRegistry` resource + basic condition evaluator (e.g. `has_keycard && power_restored`). (4 days)
  - **Completed:** `PuzzleRegistry` with `&&` / `||` / `!` / parentheses evaluator.

- [x] **[TODO-009]** Basic `Interactable` component for doors, terminals, valves with raycast interaction. (3 days)
  - **Completed:** View-ray aim + `[E]` use; HUD prompt; door open / flags / elevator / moral release actions.

- [x] **[TODO-010]** Auto-save system on elevator rides (RON serialization, 10 slots). (3 days)
  - **Completed:** Writes `saves/slot_XX.ron` on `ElevatorTransition` enter (floor, pose, flags, ending).

---

## Acceptance Criteria

- [x] 10 RON floor files exist in `assets/floors/` with layout, palette, and audio cues.
- [x] Loading a floor populates the raycaster `MapGrid` and spawns entities.
- [x] Floor clusters (Human/Hybrid/Surface) apply correct palette and faction spawns.
- [x] Elevator transition: save → unload → load → palette/audio shift.
- [x] `PuzzleRegistry` evaluates compound conditions and gates doors.
- [x] Raycast interaction works on doors, terminals, valves with HUD prompts.
- [x] Auto-save writes/loads RON save slots during elevator transitions.
