# Sprint 9: Content Completion & Balancing

**Owner:** Content + Gameplay Teams  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** Sprints 2-8 (all systems must exist before content fill + balance)

---

## Goal

Fill out all 10 floors with detailed room layouts and 3-5 interconnected puzzles
per floor. Write boss tease scripts, tune enemy waves per cluster, and do a full
weapon/enemy balance pass (ammo scarcity, puzzle vs combat ratio). This sprint
turns the system scaffolding into a playable game.

---

## Tasks

- [ ] **[TODO-035]** Detailed floor-by-floor room lists + 3–5 interconnected puzzles per floor (keycards, breakers, DNA, emails). (2 weeks)
  - **Branch:** `todo-035`
  - **Assignee:** Multiple Devs (one floor cluster each: Human 1-3, Hybrid 4-7, Surface 8-10)
  - **Dependencies:** TODO-006 (RON floor defs to fill), TODO-026 (puzzle DSL for puzzle definitions), TODO-009 (interactables), TODO-022 (hazards)
  - **Notes:** This is the big content authoring task. For each of the 10 floors:
    - **Room layout** — detailed grid map in the RON floor file (TODO-006), replacing placeholder grids with real rooms, corridors, locked doors, hazard areas.
    - **3-5 interconnected puzzles** — puzzles that depend on each other (e.g., find keycard → unlock breaker room → restore power → open elevator). Defined via the puzzle DSL (TODO-026). Puzzle types: keycards, breakers, DNA sequencers, email/terminal clues, valve timing, crate pushing, biometric doors.
    - **Interconnection** — puzzles on a floor chain together; some puzzles reference flags from previous floors (e.g., a biometric door needs a limb from floor N-1).
  - Split across devs by cluster: 3 devs for Human (1-3), 4 devs for Hybrid (4-7), 3 devs for Surface (8-10). Parallelizable.

- [ ] **[TODO-036]** Boss tease scripts (PA/terminals) + enemy wave tuning per cluster. (1 week)
  - **Branch:** `todo-036`
  - **Assignee:** Dev (content + gameplay)
  - **Dependencies:** TODO-032 (PA voice lines), TODO-017/021/025 (boss fights to tease), TODO-018 (enemy AI for wave tuning)
  - **Notes:**
    - **Boss teases** — PA announcements and terminal logs on floors leading up to each boss that foreshadow the boss (Lieutenant on Floor 3, Warden on Floor 7, Scientist on Floor 10, CEO climax). Written as RON dialogue/terminal content, played via the audio system (TODO-032) or displayed on terminals (TODO-033 egui).
    - **Enemy wave tuning** — per cluster, define enemy spawn counts, wave timing, and archetype mix in the RON floor files. Human cluster: mostly Thugs, few Enforcers. Hybrid: mix of Security + remaining Mob. Surface: Research + Executive elites.
  - This requires the audio (TODO-032) and terminal (TODO-033) systems from Sprint 8.

- [ ] **[TODO-037]** Full weapon/enemy balance pass (ammo scarcity, puzzle vs combat ratio). (1 week)
  - **Branch:** `todo-037`
  - **Assignee:** Dev (gameplay tuning)
  - **Dependencies:** TODO-013 (weapons), TODO-019 (loot drops), TODO-035 (floor content to balance against), TODO-036 (enemy waves to tune)
  - **Notes:** The core design pillar is **scarce ammo** and **puzzle vs combat balance**. Tune:
    - **Ammo drops** — reduce pistol/shotgun/plasma ammo drops so the player can't fight every enemy; must use stealth/puzzles to avoid some.
    - **Enemy health/damage** — per archetype, scaled so combat is risky but possible.
    - **Puzzle vs combat ratio** — target ~60% puzzle/exploration, ~40% combat per floor (adjust per cluster: Human more puzzle, Surface more combat).
    - **Adrenochrome Injector** — balance the health drain vs vision benefit; ensure it's useful but not spammable.
  - Iterate via playtesting (feeds into Sprint 10 TODO-038).

---

## Parallelization

```
TODO-035 (floor content) ──┬──> TODO-036 (waves + teases)
                           └──> TODO-037 (balance pass)
```

- TODO-035 is the long pole (2 weeks) and must largely complete before TODO-036 and TODO-037 can tune against real content.
- TODO-036 and TODO-037 can overlap once enough floors are authored.
- TODO-035 itself is parallelizable across devs by floor cluster.

---

## Acceptance Criteria

- [ ] All 10 floors have detailed room layouts in RON.
- [ ] Each floor has 3-5 interconnected puzzles defined via DSL.
- [ ] Boss tease scripts play on approach floors (PA/terminals).
- [ ] Enemy waves are tuned per cluster in RON floor files.
- [ ] Ammo scarcity forces puzzle/stealth use; combat is risky.
- [ ] Puzzle vs combat ratio feels right per cluster (~60/40).
