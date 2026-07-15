# Cross-Cutting / Global TODOs

**Owner:** Assign as needed  
**Status:** Not started  
**Dependencies:** None (these support all sprints)

---

These tasks are not bound to a single sprint. They support the entire project
and should be started early and maintained throughout.

## Tasks

- [ ] **[TODO-042]** Comprehensive asset pipeline (sprites in RON/aseprite format, audio import workflow).
  - **Branch:** `todo-042`
  - **Assignee:** Art Team + Dev (pipeline)
  - **Notes:** The current `assets/` directory only has `fonts/FiraSans-Bold.ttf`. This task sets up:
    - **Sprite pipeline** — Aseprite files exported to PNG + RON metadata (frame indices, animation timings, billboard dimensions). A build script or Bevy asset loader that reads Aseprite exports.
    - **Audio pipeline** — import workflow for tracker music (MIDI/mod), SFX (wav/ogg), PA voice lines. Naming conventions and folder structure (`assets/sprites/`, `assets/audio/music/`, `assets/audio/sfx/`, `assets/audio/voice/`).
    - **Floor RON validation** — a script or test that validates floor RON files against the `FloorDef` schema (TODO-006).
  - This unblocks TODO-011 (hand sprites), TODO-016/020/024/028 (enemy sprites), TODO-032 (audio).

- [ ] **[TODO-043]** Project documentation: enemy hierarchy tables, puzzle prereq reference, Bevy component cheatsheet.
  - **Branch:** `todo-043`
  - **Assignee:** Dev (docs)
  - **Notes:** This `docs/sprints/` structure is the start. Extend with:
    - **Enemy hierarchy table** — faction, archetype, floor cluster, health, damage, AI behavior, loot table.
    - **Puzzle prerequisite reference** — which puzzles on which floors depend on which flags, in dependency order.
    - **Bevy component cheatsheet** — all custom components (`Player`, `Enemy`, `Interactable`, `PuzzleInteractable`, etc.) with their fields and which systems read/write them.
  - The scaffold snapshot table in `docs/sprints/README.md` is a starting point.

- [ ] **[TODO-044]** Git workflow setup: feature branches per TODO-XXX, team code reviews, CI basics.
  - **Branch:** `todo-044`
  - **Assignee:** Lead Dev / DevOps
  - **Notes:**
    - **Feature branches** — `todo-XXX` naming convention (already specified in the master doc).
    - **PR reviews** — Master Devs review ECS patterns; require `cargo check` + `cargo clippy` passing.
    - **CI** — GitHub Actions (or equivalent): `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `cargo build --release`. Run on all PRs.
    - **Branch protection** — require review + CI green before merge to `main`.

---

## Acceptance Criteria

- [ ] Asset pipeline: Aseprite → PNG + RON, audio import workflow, floor RON validation.
- [ ] Documentation: enemy hierarchy, puzzle prereqs, component cheatsheet.
- [ ] Git workflow: feature branches, PR reviews, CI (fmt + clippy + test + build).
