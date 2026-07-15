# Sprint 7: Executive Tier, Progression & Moral Choice

**Owner:** Content + Systems Teams  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** Sprint 4-6 (all faction tiers), Sprint 2 (puzzle registry), Sprint 3 (inventory)

---

## Goal

Build the final faction tier: the CEO's Executive forces. Implement the moral
choice system that tracks player decisions across all floors, the two endings
(subjects released vs. not), permanent mutation perks (progression), and the
side-puzzle system that feeds the moral choices.

---

## Tasks

- [ ] **[TODO-028]** Executive faction: Bodyguards, Admin Secretaries, Limo Drivers (stealth alarms, ending convoy). (6 days)
  - **Branch:** `todo-028`
  - **Assignee:** Dev (art + gameplay)
  - **Dependencies:** TODO-015 (Enemy bundle + faction enum), TODO-018 (grunt AI to extend)
  - **Notes:** 3 archetypes for the Executive tier:
    - **Bodyguards** — high health, high damage, may escort the CEO. Frontal shields like Riot Guards.
    - **Admin Secretaries** — non-combat or light combat, trigger stealth alarms (alert all enemies on the floor). Moral choice targets (innocent workers?).
    - **Limo Drivers** — appear in the ending convoy sequence (narrative/escort or chase).
  - Stealth alarms: a floor-wide alert system (distinct from per-enemy detection) that Admin Secretaries trigger — raises a floor alarm flag that spawns waves or locks doors. These spawn on floors 8-10 (Surface cluster, overlapping with Research faction — Executives are the elite guard layer).

- [ ] **[TODO-029]** CEO narrative climax + two endings (subjects_released flag from side-puzzles). (1 week)
  - **Branch:** `todo-029`
  - **Assignee:** Dev (narrative + systems)
  - **Dependencies:** TODO-031 (side-puzzle moral choice tracking), TODO-028 (Executive faction), TODO-005 (`Ending` state)
  - **Notes:** The CEO is the final boss (narrative climax, not necessarily a combat fight — could be a moral choice confrontation). Two endings based on the `subjects_released` flag:
    - **Ending A (subjects released):** Player chose to release the test subjects across floors (via side-puzzles). CEO is confronted/exposed. "Good" ending — snowy mountain road with limos (TODO-040 cinematic).
    - **Ending B (subjects not released):** Player chose self-interest / escaped without freeing subjects. CEO escapes. "Bad" ending.
  - The `subjects_released` flag is set by the side-puzzle system (TODO-031) and tracked in the `PuzzleRegistry` / save system. The `Ending` game state (TODO-005) branches into the two cinematics.

- [ ] **[TODO-030]** Permanent mutation perks (one every 3 floors: speed, inventory, night vision). (4 days)
  - **Branch:** `todo-030`
  - **Assignee:** Dev
  - **Dependencies:** TODO-012 (player components), TODO-007 (floor progression to track every-3-floors)
  - **Notes:** Progression system: every 3 floors (Floor 3, 6, 9), the player gains a permanent mutation perk:
    - **Floor 3:** Speed (faster movement).
    - **Floor 6:** Inventory (extra slots).
    - **Floor 9:** Night vision (enhanced dark-floor visibility — ties to the Adrenochrome Injector's vision mechanic).
  - Perks are permanent (persist across saves) and modify player components. The Adrenochrome Injector's temp vision (TODO-013) and the night vision perk (TODO-030) share the vision system — the perk is a passive version.

- [ ] **[TODO-031]** Side-puzzle system for tracking moral choices across floors. (5 days)
  - **Branch:** `todo-031`
  - **Assignee:** Dev
  - **Dependencies:** TODO-026 (puzzle DSL for side-puzzle definitions), TODO-008 (PuzzleRegistry for flag tracking), TODO-010 (save system to persist choices)
  - **Notes:** Side-puzzles are optional puzzles on each floor that present moral choices (e.g., release test subjects, spare defeated faction members, destroy or preserve research data). Each choice sets a flag in the `PuzzleRegistry` (via the DSL effect system from TODO-026). The cumulative flags determine the ending (TODO-029). The `FactionRegistry` (TODO-023) feeds into this: did the player spare or execute defeated faction bosses? Side-puzzles are defined in RON via the DSL and loaded per floor.

---

## Parallelization

```
TODO-028 ──┐
TODO-030 ──┼──> TODO-029 (endings need faction + moral flags)
TODO-031 ──┘
```

- TODO-028, TODO-030, and TODO-031 can run in parallel (faction vs perks vs moral tracking).
- TODO-029 (endings) depends on all three: Executive faction for the climax, moral flags for the branch, perks for the progression context.

---

## Acceptance Criteria

- [ ] 3 Executive archetypes spawn with stealth alarm and escort behavior.
- [ ] Two endings branch on `subjects_released` flag.
- [ ] Mutation perks granted at floors 3, 6, 9 (speed, inventory, night vision).
- [ ] Side-puzzles track moral choices via DSL flags, persisted in saves.
- [ ] CEO narrative climax triggers the appropriate ending cinematic.
