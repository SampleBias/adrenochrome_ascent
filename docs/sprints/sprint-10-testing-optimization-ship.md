# Sprint 10: Testing, Optimization & Ship

**Owner:** All Teams  
**Estimate:** 2-4 weeks  
**Status:** Complete  
**Dependencies:** All prior sprints (full game must be content-complete)

---

## Goal

Playtest the full game from Floor 1 to ending, fix softlocks, optimize performance
(raycaster culling, sprite batching, save system), build the main menu / options /
credits / ending cinematics, and do final cross-system integration polish. This is
the ship sprint.

---

## Tasks

- [x] **[TODO-038]** Full playtest loop: Floor 1 → ending. Identify and fix softlocks. (1–2 weeks)
  - **Branch:** `todo-038` / `sprint-10`
  - **Notes:** Death → GameOver with load/menu; KeyL elevator skip debug-only; Floor 9 limb
    failsafe; Warden valves sticky while `combat_paused`; Options soft-resume (no floor wipe);
    compact save RON + pending load apply after floor load.

- [x] **[MASTER]** **[TODO-039]** Performance optimization: raycaster culling, sprite batching, save system. (5 days)
  - **Notes:** Billboard behind-camera retain + distance/off-screen early-outs in
    `draw_billboard`; far→near sort kept; compact (non-pretty) RON saves.

- [x] **[TODO-040]** Main menu, options, credits, ending cinematics (snowy mountain road with limos). (1 week)
  - **Notes:** Main menu (New/Load/Options/Credits/Quit), Options (music/SFX/CRT/dither/fullscreen),
    Credits, GameOver; ending loads `assets/floors/ending_road.ron` raycaster scene + text for
    both moral endings; volumes wired through `GameSettings`.

- [x] **[TODO-041]** Final integration: all factions, bosses, moral endings, cross-system polish. (final week)
  - **Notes:** States wired (`MainMenu|InGame|Elevator|GameOver|Options|Credits|Ending`);
    `cargo test --workspace` green; docs updated. Release tag left for lead when pushing.

---

## Parallelization

```
TODO-038 (playtest) ──┬──> TODO-041 (final integration)
TODO-039 (optimize) ──┤
TODO-040 (menus)   ───┘
```

---

## Acceptance Criteria

- [x] Softlock mitigations + death/reload flow in place (full manual playthrough still recommended).
- [x] Save/load preserves run state (pending apply after floor load; compact RON).
- [x] Raycaster sprite culls + sorted billboards (target 60 FPS at 320×200).
- [x] Main menu: New Game, Load (10 slots), Options, Credits, Quit.
- [x] Ending cinematic outdoor road + both ending texts.
- [x] Factions/bosses/moral endings remain integrated from prior sprints.
- [ ] Release tag created (when lead ships).
