# Sprint 10: Testing, Optimization & Ship

**Owner:** All Teams  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** All prior sprints (full game must be content-complete)

---

## Goal

Playtest the full game from Floor 1 to ending, fix softlocks, optimize performance
(raycaster culling, sprite batching, save system), build the main menu / options /
credits / ending cinematics, and do final cross-system integration polish. This is
the ship sprint.

---

## Tasks

- [ ] **[TODO-038]** Full playtest loop: Floor 1 → ending. Identify and fix softlocks. (1–2 weeks)
  - **Branch:** `todo-038`
  - **Assignee:** All Devs (split by floor cluster for softlock hunting)
  - **Dependencies:** TODO-035 (all floor content), TODO-029 (both endings reachable), TODO-010 (save/load working)
  - **Notes:** Play through the entire game (both ending branches). Look for:
    - **Softlocks** — puzzles that can be put into an unsolvable state (e.g., used a keycard on the wrong door, can't progress). Fix by adding reset mechanics or preventing bad states.
    - **Sequence breaks** — reaching floors/puzzles out of order. Decide if intentional (speedrun-friendly) or fix.
    - **Save/load bugs** — load a save and verify all state (puzzle flags, inventory, faction defeats, moral choices) restores correctly.
    - **Boss fight bugs** — bosses that can be skipped, stuck, or crash.
  - Log issues as bugs; fix in feature branches. This is iterative with TODO-037 (balance).

- [ ] **[MASTER]** **[TODO-039]** Performance optimization: raycaster culling, sprite batching, save system. (5 days)
  - **Branch:** `todo-039`
  - **Assignee:** Master Dev
  - **Dependencies:** TODO-003 (raycaster to optimize), TODO-010 (save system to optimize), TODO-035 (full content to stress-test)
  - **Notes:**
    - **Raycaster culling** — only cast rays for visible columns; skip off-screen sprites; frustum cull entities behind the player. The DDA loop is the main CPU cost — optimize the inner loop.
    - **Sprite batching** — batch billboard sprite rendering (enemies, items, hand) into fewer draw calls. Sort by distance for correct overdraw.
    - **Save system** — RON serialization can be slow for large state; profile and optimize (lazy serialization, delta saves, or switch to a faster format if needed).
    - **Profiling** — use `puffin` or `tracy` to find hotspots. Target 60 FPS at 320×200 internal resolution.
  - Reserved for Master Dev (deep engine knowledge required).

- [ ] **[TODO-040]** Main menu, options, credits, ending cinematics (snowy mountain road with limos). (1 week)
  - **Branch:** `todo-040`
  - **Assignee:** Dev (UI + narrative)
  - **Dependencies:** TODO-005 (`MainMenu`/`Ending` states), TODO-029 (two endings), TODO-033 (UI system), TODO-010 (save slots for menu load)
  - **Notes:** The existing `src/ui/menus.rs` has a basic main menu, pause, game over, and victory screen using `TextBundle`. Extend into:
    - **Main menu** — New Game, Load Game (10 save slots from TODO-010), Options, Credits, Quit. Pixel font, CRT aesthetic.
    - **Options** — video (resolution, fullscreen), audio (music/SFX volume), controls (rebind), accessibility (CRT effects toggle, dither toggle).
    - **Credits** — team credits, scroll.
    - **Ending cinematics** — two endings (TODO-029): snowy mountain road with limos (Ending A: subjects released, hopeful), and the darker alternative (Ending B). Render as a sequence of sprites/text over the CRT pipeline, or a simple raycaster scene (outdoor road).
  - The ending cinematic ("snowy mountain road with limos") is a unique raycaster scene — the only outdoor non-lab environment.

- [ ] **[TODO-041]** Final integration: all factions, bosses, moral endings, cross-system polish. (final week)
  - **Branch:** `todo-041`
  - **Assignee:** All Devs (integration)
  - **Dependencies:** All TODOs (this is the final integration pass)
  - **Notes:** The final week. Verify all systems work together:
    - All 4 factions spawn correctly per floor cluster.
    - All 4 bosses (Lieutenant, Warden, Scientist, CEO) trigger and are beatable.
    - Both endings are reachable based on moral choices.
    - Save/load preserves all state across a full run.
    - Audio, UI, post-processing all work in all states.
    - No crashes, no softlocks, stable 60 FPS.
  - This is the "are we done?" pass. Tag a release commit when complete.

---

## Parallelization

```
TODO-038 (playtest) ──┬──> TODO-041 (final integration)
TODO-039 (optimize) ──┤
TODO-040 (menus)   ───┘
```

- TODO-038, TODO-039, and TODO-040 can run in parallel (playtesting, optimization, menu/cinematic work).
- TODO-041 (final integration) is the gate — it depends on everything being stable.

---

## Acceptance Criteria

- [ ] Full playthrough Floor 1 → both endings works without softlocks.
- [ ] Save/load preserves all state across a full run.
- [ ] 60 FPS at 320×200 internal resolution with full content.
- [ ] Main menu: New Game, Load (10 slots), Options, Credits, Quit.
- [ ] Both ending cinematics play correctly.
- [ ] All factions, bosses, moral choices integrated and working.
- [ ] Release tag created.
