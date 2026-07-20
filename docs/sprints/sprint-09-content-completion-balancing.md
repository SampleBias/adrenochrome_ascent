# Sprint 9: Content Completion & Balancing

**Owner:** Content + Gameplay Teams  
**Estimate:** 2-4 weeks  
**Status:** Complete  
**Dependencies:** Sprints 2-8 (all systems must exist before content fill + balance)

---

## Goal

Fill out all 10 floors with detailed room layouts and 3-5 interconnected puzzles
per floor. Write boss tease scripts, tune enemy waves per cluster, and do a full
weapon/enemy balance pass (ammo scarcity, puzzle vs combat ratio).

---

## Tasks

- [x] **[TODO-035]** Detailed floor-by-floor room lists + 3–5 interconnected puzzles per floor.
  - All `assets/floors/floor_XX.ron` rewritten with denser grids and chained flags.
  - Room/puzzle reference: `docs/content/floor-room-lists.md`.

- [x] **[TODO-036]** Boss tease scripts (PA/terminals) + enemy wave tuning per cluster.
  - `AnnouncePa` interact action; approach-floor terminals/PA for LT / Warden / Scientist-CEO.
  - `WaveTuning` on `FloorDef` → `ActiveWaveTuning` drives Lieutenant / Warden summons.

- [x] **[TODO-037]** Full weapon/enemy balance pass (ammo scarcity, puzzle vs combat ratio).
  - Lower start ammo / loot; slower plasma & injector; tougher archetypes.
  - Tables: `docs/content/balance-tables.md`.

---

## Acceptance Criteria

- [x] All 10 floors have detailed room layouts in RON.
- [x] Each floor has 3-5 interconnected puzzles defined via DSL.
- [x] Boss tease scripts play on approach floors (PA/terminals).
- [x] Enemy waves are tuned per cluster in RON floor files.
- [x] Ammo scarcity forces puzzle/stealth use; combat is risky.
- [x] Puzzle vs combat ratio skewed puzzle-heavy early, combat-heavier late.
