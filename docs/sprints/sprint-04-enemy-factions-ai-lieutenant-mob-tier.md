# Sprint 4: Enemy Factions & AI (Lieutenant Mob Tier)

**Owner:** Gameplay + Art Teams  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** Sprint 1 (raycaster + billboard sprites), Sprint 2 (floor loader spawns enemies), Sprint 3 (player weapons for combat)

---

## Goal

Build the enemy system foundation and the first faction tier: the Lieutenant's
Mob. This covers the base `Enemy` bundle, faction enum, Mob-tier archetypes, the
Lieutenant boss fight (Floor 3), grunt AI state machine, and loot drops. This
sprint replaces the current single `Scientist` patrol component with a full
faction system.

---

## Tasks

- [ ] **[TODO-015]** Base `Enemy` bundle + faction enum (Mob, Security, Research, Executive). (3 days)
  - **Branch:** `todo-015`
  - **Assignee:** Dev
  - **Dependencies:** TODO-003 (billboard sprites), TODO-007 (floor loader spawns enemies)
  - **Notes:** The existing `src/enemy/components.rs` has a `Scientist` component with waypoints + speed. Replace with a general `Enemy` bundle: `Health`, `Faction` (enum: Mob, Security, Research, Executive), `EnemyState` (Patrol/Chase/Attack/Dead), `Sprite`, `Transform`, `AI` data. The `Faction` enum maps directly to the boss hierarchy tiers. The existing `SecurityCamera` component can stay as an environmental hazard, not an enemy. The `PlayerDetected` event is reusable.

- [ ] **[TODO-016]** Lieutenant faction sprites & archetypes: Foot Soldier Thugs, Enforcer Heavies, Zed Prisoners. (1 week)
  - **Branch:** `todo-016`
  - **Assignee:** Dev (art integration)
  - **Dependencies:** TODO-015 (Enemy bundle), TODO-003 (billboard sprites)
  - **Notes:** 3 archetypes for the Mob tier:
    - **Foot Soldier Thugs** — basic melee/chase grunts, low health, common.
    - **Enforcer Heavies** — slow, high health, high damage, may have shields.
    - **Zed Prisoners** — fast, erratic, low health, swarm behavior.
  - Each needs billboard sprite(s) (idle, walk, attack, death frames). Flag art creation for TODO-042. Use placeholder colored billboards until art is ready. These spawn on floors 1-3 (Human cluster).

- [ ] **[MASTER]** **[TODO-017]** Lieutenant boss fight (Floor 3): wave summoning, cigar weakpoint, flooded cell arena logic. (1 week)
  - **Branch:** `todo-017`
  - **Assignee:** Master Dev
  - **Dependencies:** TODO-015 (Enemy bundle), TODO-016 (Mob archetypes for wave summons), TODO-007 (Floor 3 loaded), TODO-013 (weapons to fight boss)
  - **Notes:** Boss arena on Floor 3 (flooded cell block). Mechanics:
    - **Wave summoning** — Lieutenant summons Mob archetypes (TODO-016) in waves.
    - **Cigar weakpoint** — the Lieutenant's lit cigar is the damage target; hitting it stuns/progresses the fight. Requires precise raycast hitscan aim.
    - **Flooded cell arena** — rising water hazard (environmental, ties to TODO-022 hazard system in Sprint 5). Player must manage positioning while fighting.
  - This is a complex multi-system boss — reserved for Master Dev. Define a `BossFight` resource/component to manage phase state.

- [ ] **[TODO-018]** Simple ECS behavior tree / state machine for grunts (Patrol → Chase → Attack). (5 days)
  - **Branch:** `todo-018`
  - **Assignee:** Dev
  - **Dependencies:** TODO-015 (Enemy bundle + EnemyState), TODO-003 (raycaster for LOS/vision)
  - **Notes:** The existing `src/enemy/systems.rs` has `scientist_patrol` (waypoint cycling) and `detection_check` (FOV-based detection). Generalize into a state machine: **Patrol** (waypoint cycling, existing logic) → **Chase** (move toward player on detection) → **Attack** (in range, deal damage). Use the raycaster for line-of-sight checks (can the grunt see the player through walls?). The `DetectionMeter` resource from `src/game/plugin.rs` can be repurposed or replaced with per-enemy detection state.

- [ ] **[TODO-019]** Loot drop system tied to factions (ammo, health, key items). (3 days)
  - **Branch:** `todo-019`
  - **Assignee:** Dev
  - **Dependencies:** TODO-015 (faction enum), TODO-012 (inventory to receive loot), TODO-013 (ammo types)
  - **Notes:** Enemies drop loot on death based on faction: Mob drops pistol/shotgun ammo + occasional health; Security drops different ammo; etc. Loot spawns as a pickup billboard sprite the player walks over or interacts with. Ammo scarcity tuning is TODO-037. Key items (keycards) can be guaranteed drops from specific enemies or boss completion.

---

## Parallelization

```
TODO-015 ──┬──> TODO-016 ──┐
           ├──> TODO-018   ├──> TODO-017 (boss, needs archetypes + AI)
           └──> TODO-019   ┘
```

- TODO-015 must complete first (base Enemy bundle).
- TODO-016, TODO-018, TODO-019 can run in parallel after TODO-015.
- TODO-017 (boss) depends on TODO-015, TODO-016, and ideally TODO-018 (grunts for waves).

---

## Acceptance Criteria

- [ ] `Enemy` bundle with `Faction` enum (Mob, Security, Research, Executive) exists.
- [ ] 3 Mob archetypes spawn and render as billboard sprites.
- [ ] Grunt AI cycles Patrol → Chase → Attack with raycaster LOS.
- [ ] Lieutenant boss fight on Floor 3: waves, cigar weakpoint, flooded arena.
- [ ] Enemies drop faction-appropriate loot (ammo, health, key items).
- [ ] Player weapons (Sprint 3) damage and kill enemies.
