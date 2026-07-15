# Sprint 5: Mid-Game Factions & Hazards (Warden Tier)

**Owner:** Gameplay + Engine Teams  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** Sprint 4 (Enemy bundle, faction enum, grunt AI, loot system)

---

## Goal

Build the second faction tier: the Warden's Security forces. Introduce
environmental hazards (valves, crate pushing), the Warden boss fight (Floor 7)
with mid-fight valve puzzles and flood hazards, and the faction despawn/registry
system that cleans up enemies when a boss is defeated.

---

## Tasks

- [ ] **[TODO-020]** Warden faction: Riot Guards, Patrol Security, Hazard Techs (shields, turrets, radio AI). (1 week)
  - **Branch:** `todo-020`
  - **Assignee:** Dev (art + gameplay)
  - **Dependencies:** TODO-015 (Enemy bundle + faction enum), TODO-018 (grunt AI to extend)
  - **Notes:** 3 archetypes for the Security tier:
    - **Riot Guards** — shielded, slow, must be flanked or shot from behind. Shield blocks raycast hitscan from the front.
    - **Patrol Security** — standard patrol/chase grunts, medium health, radio alerts (calling other enemies).
    - **Hazard Techs** — deploy turrets (stationary raycast emitters), support role.
  - Radio AI: Patrol Security can alert nearby enemies (trigger Chase state) when they detect the player. Turrets: a stationary entity that fires hitscan rays at the player on LOS. These spawn on floors 4-7 (Hybrid cluster). The `SecurityCamera` component from the existing scaffold can be repurposed for turret vision.

- [ ] **[MASTER]** **[TODO-021]** Warden boss (Floor 7): mid-fight valve puzzles, flood hazards via `WardenOverrides` resource. (1 week)
  - **Branch:** `todo-021`
  - **Assignee:** Master Dev
  - **Dependencies:** TODO-020 (Warden faction), TODO-022 (hazard system), TODO-007 (Floor 7 loaded), TODO-013 (weapons)
  - **Notes:** Boss arena on Floor 7. Mechanics:
    - **Mid-fight valve puzzles** — the fight pauses combat waves; player must solve valve puzzles (TODO-022) to progress phases. The `PuzzleRegistry` (TODO-008) gates phase transitions.
    - **Flood hazards** — rising/falling water levels in the arena, managed by a `WardenOverrides` resource that the boss controls. Player must manage positioning.
    - **`WardenOverrides` resource** — a boss-specific resource that overrides the hazard system (TODO-022) during the fight, e.g. forcing valve states or flood levels.
  - Complex multi-system boss — reserved for Master Dev.

- [ ] **[TODO-022]** Environmental hazards: timed valves, grid-based crate/forklift pushing. (5 days)
  - **Branch:** `todo-022`
  - **Assignee:** Dev
  - **Dependencies:** TODO-009 (Interactable component for valves), TODO-003 (raycaster grid for crate pushing)
  - **Notes:**
    - **Timed valves** — interactable valves (via TODO-009 `Interactable`) that must be turned within a time window to open doors / stop floods. Uses the `TimingPuzzle` component pattern from the existing `src/puzzle/components.rs`.
    - **Grid-based crate/forklift pushing** — the raycaster's `MapGrid` defines pushable cells. Player walks into a crate to push it one cell (Doom-style), solving spatial puzzles (block a flood, reach a switch). This is a grid-state puzzle, not physics-based.
  - These hazards appear on floors 4-7 (Hybrid cluster) and in the Warden boss arena.

- [ ] **[TODO-023]** Faction despawn on boss defeat + global `FactionRegistry` resource. (3 days)
  - **Branch:** `todo-023`
  - **Assignee:** Dev
  - **Dependencies:** TODO-015 (faction enum), TODO-017 (Lieutenant boss defeat to test with), TODO-021 (Warden boss defeat)
  - **Notes:** When a boss is defeated, all enemies of that faction on the current floor despawn (the area is "cleared"). A global `FactionRegistry` resource tracks which factions have been defeated across the run — used to prevent respawns and gate progression. This also feeds the moral-choice system (Sprint 7): did the player spare or execute defeated faction members?

---

## Parallelization

```
TODO-020 ──┐
TODO-022 ──┼──> TODO-021 (boss, needs faction + hazards)
           └──> TODO-023 (needs boss defeats to test)
```

- TODO-020 and TODO-022 can run in parallel (faction archetypes vs hazard system).
- TODO-021 (boss) depends on both TODO-020 and TODO-022.
- TODO-023 depends on at least one boss being defeat-able (TODO-017 from Sprint 4 or TODO-021).

---

## Acceptance Criteria

- [ ] 3 Security archetypes (Riot Guard, Patrol Security, Hazard Tech) spawn and fight.
- [ ] Riot Guard shields block frontal hitscan; turrets fire at player.
- [ ] Radio AI: Patrol Security alerts nearby enemies.
- [ ] Warden boss on Floor 7: valve puzzle phases, flood hazards, `WardenOverrides`.
- [ ] Timed valves and grid-based crate pushing work as puzzles.
- [ ] Boss defeat despawns that faction's enemies; `FactionRegistry` tracks defeats.
