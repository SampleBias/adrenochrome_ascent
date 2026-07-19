# Sprint Index — Adrenochrome Ascent

This directory contains the per-sprint breakdown of the [Master TODO List](../master_todo.md).
Each sprint file is self-contained: goal, owner, tasks with trackable checkboxes,
dependencies, parallelization notes, and mapping to the existing codebase scaffold.

## Sprints

| # | Sprint | File | Tasks | Status |
|---|--------|------|-------|--------|
| 1 | Project Bootstrap & Core Engine Foundation | [sprint-01-project-bootstrap-core-engine-foundation.md](sprint-01-project-bootstrap-core-engine-foundation.md) | TODO-001 → 005 | Complete |
| 2 | Map & Floor Loading System | [sprint-02-map-floor-loading-system.md](sprint-02-map-floor-loading-system.md) | TODO-006 → 010 | Complete |
| 3 | Player Systems & Inventory | [sprint-03-player-systems-inventory.md](sprint-03-player-systems-inventory.md) | TODO-011 → 014 | Complete |
| 4 | Enemy Factions & AI (Lieutenant Mob Tier) | [sprint-04-enemy-factions-ai-lieutenant-mob-tier.md](sprint-04-enemy-factions-ai-lieutenant-mob-tier.md) | TODO-015 → 019 | Complete |
| 5 | Mid-Game Factions & Hazards (Warden Tier) | [sprint-05-mid-game-factions-hazards-warden-tier.md](sprint-05-mid-game-factions-hazards-warden-tier.md) | TODO-020 → 023 | Complete |
| 6 | Late-Game Factions & Mini-Games (Scientist Tier) | [sprint-06-late-game-factions-minigames-scientist-tier.md](sprint-06-late-game-factions-minigames-scientist-tier.md) | TODO-024 → 027 | Not started |
| 7 | Executive Tier, Progression & Moral Choice | [sprint-07-executive-tier-progression-moral-choice.md](sprint-07-executive-tier-progression-moral-choice.md) | TODO-028 → 031 | Not started |
| 8 | Audio, UI & Polish | [sprint-08-audio-ui-polish.md](sprint-08-audio-ui-polish.md) | TODO-032 → 034 | Not started |
| 9 | Content Completion & Balancing | [sprint-09-content-completion-balancing.md](sprint-09-content-completion-balancing.md) | TODO-035 → 037 | Not started |
| 10 | Testing, Optimization & Ship | [sprint-10-testing-optimization-ship.md](sprint-10-testing-optimization-ship.md) | TODO-038 → 041 | Not started |

## Cross-Cutting TODOs

Global tasks not bound to a single sprint: [sprint-00-cross-cutting.md](sprint-00-cross-cutting.md) (TODO-042 → 044).

## How to Use These Files

1. **Pick a sprint** — sprints are ordered by dependency, but tasks within and across
   sprints can be parallelized (noted in each file).
2. **Check the dependencies** — each sprint lists what must be done first.
3. **Claim a task** — create a feature branch `todo-XXX` and open a PR when ready.
4. **Track progress** — check the box `- [x]` in both the sprint file and the
   [master TODO list](../master_todo.md) when a task is complete.
5. **[MASTER] tasks** are reserved for the 2 Master Devs (raycaster, DSL parser,
   boss AI, systems integration).

## Legend

- `[ ]` — not started
- `[~]` — in progress
- `[x]` — complete
- **[MASTER]** — reserved for Master Devs
- `todo-XXX` — feature branch name for that task

## Project Snapshot (as of scaffold evaluation)

The current codebase is a **Bevy 0.14 3D first-person scaffold** with 7 hardcoded
levels. The master TODO list calls for a **Bevy 0.15+ raycaster** with 10 floors,
CRT aesthetic, combat, and a puzzle DSL. **Sprint 1 is a migration + rewrite sprint**
before content work can begin.

| Scaffold module | Current state | Relevant sprint |
|-----------------|--------------|-----------------|
| `src/game/states.rs` | 7-level enum (Level1..7, Paused, GameOver, Victory) | Sprint 1 (TODO-005) |
| `src/game/constants.rs` | 3D movement + detection constants | Sprint 1, 3 |
| `src/player/controller.rs` | Kinematic 3D FPS controller (no physics) | Sprint 1 (TODO-004) |
| `src/level/definitions.rs` | Static 7-level definitions | Sprint 2 (TODO-006) |
| `src/level/loader.rs` | Mesh-based PbrBundle walls/floors | Sprint 1, 2 (TODO-003, 007) |
| `src/puzzle/components.rs` | PuzzleInteractable, Keypad, Timing, Circuit | Sprint 2, 6 (TODO-008, 026) |
| `src/puzzle/systems.rs` | Distance-based interaction, stub solve | Sprint 2 (TODO-009) |
| `src/enemy/components.rs` | Scientist patrol, SecurityCamera sweep | Sprint 4 (TODO-015) |
| `src/enemy/systems.rs` | Patrol + FOV detection | Sprint 4 (TODO-018) |
| `src/ui/hud.rs` | Crosshair, detection bar, subtitle | Sprint 8 (TODO-033) |
| `src/ui/menus.rs` | Main menu, pause, game over, victory | Sprint 10 (TODO-040) |
| `src/audio/plugin.rs` | Empty stub | Sprint 8 (TODO-032) |
