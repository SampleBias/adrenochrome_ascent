# Sprint 5: Mid-Game Factions & Hazards (Warden Tier)

**Owner:** Gameplay + Engine Teams  
**Estimate:** 2-4 weeks  
**Status:** Complete  
**Dependencies:** Sprint 4 (Enemy bundle, faction enum, grunt AI, loot system)

---

## Goal

Build the second faction tier: the Warden's Security forces. Introduce
environmental hazards (valves, crate pushing), the Warden boss fight (Floor 7)
with mid-fight valve puzzles and flood hazards, and the faction despawn/registry
system that cleans up enemies when a boss is defeated.

---

## Tasks

- [x] **[TODO-020]** Warden faction: Riot Guards, Patrol Security, Hazard Techs (shields, turrets, radio AI).
  - **Branch:** `sprint-05`
  - **Notes:** Archetypes + sprites 22–29. Frontal shield blocks hitscan. Radio alerts allies. Hazard Techs deploy turrets; RON `Turret` entities also supported.

- [x] **[MASTER]** **[TODO-021]** Warden boss (Floor 7): mid-fight valve puzzles, flood hazards via `WardenOverrides`.
  - **Branch:** `sprint-05`
  - **Notes:** HP thresholds pause combat; player must hit timed valves A/B. `WardenOverrides` drives flood DPS. Elevator gated on `warden_down && basin_open`.

- [x] **[TODO-022]** Environmental hazards: timed valves, grid-based crate/forklift pushing.
  - **Branch:** `sprint-05`
  - **Notes:** `InteractAction::TimedValve`, `TimedValveState`, `PushableCrate` + walk-into push. Floor 6 uses a 12s coolant window.

- [x] **[TODO-023]** Faction despawn on boss defeat + global `FactionRegistry` resource.
  - **Branch:** `sprint-05`
  - **Notes:** Boss death marks faction; living allies (+ turrets for Security) despawn. Defeated factions skip respawn on floor load. Clears on main menu.

---

## Acceptance Criteria

- [x] 3 Security archetypes (Riot Guard, Patrol Security, Hazard Tech) spawn and fight.
- [x] Riot Guard shields block frontal hitscan; turrets fire at player.
- [x] Radio AI: Patrol Security alerts nearby enemies.
- [x] Warden boss on Floor 7: valve puzzle phases, flood hazards, `WardenOverrides`.
- [x] Timed valves and grid-based crate pushing work as puzzles.
- [x] Boss defeat despawns that faction's enemies; `FactionRegistry` tracks defeats.

---

## Playtest notes

| Floor | Highlights |
|-------|------------|
| 4 | Patrol + Riot Guard; pushable crate |
| 5 | Hazard Tech (deploys turret) + static turrets |
| 6 | Timed coolant valve (12s) → race the lock |
| 7 | Warden — watch HUD for valve phases; flank shields |

Flank Riot Guards / Warden (shields block frontal shots).
