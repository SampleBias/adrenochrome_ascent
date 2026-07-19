# Sprint 4: Enemy Factions & AI (Lieutenant Mob Tier)

**Owner:** Gameplay + Art Teams  
**Estimate:** 2-4 weeks  
**Status:** Complete  
**Dependencies:** Sprint 1 (raycaster + billboard sprites), Sprint 2 (floor loader spawns enemies), Sprint 3 (player weapons for combat)

---

## Goal

Build the enemy system foundation and the first faction tier: the Lieutenant's
Mob. This covers the base `Enemy` bundle, faction enum, Mob-tier archetypes, the
Lieutenant boss fight (Floor 3), grunt AI state machine, and loot drops.

---

## Tasks

- [x] **[TODO-015]** Base `Enemy` bundle + faction enum (Mob, Security, Research, Executive).
  - **Branch:** `sprint-04`
  - **Notes:** `gameplay::enemy` — `Enemy`, `Faction`, `EnemyArchetype`, `EnemyAi`, `EnemyState`. Content `EntityKind::Enemy` in RON.

- [x] **[TODO-016]** Lieutenant faction sprites & archetypes: Foot Soldier Thugs, Enforcer Heavies, Zed Prisoners.
  - **Branch:** `sprint-04`
  - **Notes:** Procedural sprites ids 12–19. Archetype stats in `enemy/archetype.rs`. Floors 1–2 spawn all three.

- [x] **[MASTER]** **[TODO-017]** Lieutenant boss fight (Floor 3): wave summoning, cigar weakpoint, flooded cell arena logic.
  - **Branch:** `sprint-04`
  - **Notes:** `BossFight` resource + `LieutenantBoss`. Cigar vulnerability windows, wave summons, flood DPS in arena basin. Elevator requires `vault_open && lieutenant_down`.

- [x] **[TODO-018]** Simple ECS behavior tree / state machine for grunts (Patrol → Chase → Attack).
  - **Branch:** `sprint-04`
  - **Notes:** LOS via `cast_ray`. Melee damage applies `apply_damage` + pain flash.

- [x] **[TODO-019]** Loot drop system tied to factions (ammo, health, key items).
  - **Branch:** `sprint-04`
  - **Notes:** Walk-over `LootPickup` billboards. Lieutenant drops `lieutenant_down` flag + keycard.

---

## Parallelization

```
TODO-015 ──┬──> TODO-016 ──┐
           ├──> TODO-018   ├──> TODO-017 (boss, needs archetypes + AI)
           └──> TODO-019   ┘
```

---

## Acceptance Criteria

- [x] `Enemy` bundle with `Faction` enum (Mob, Security, Research, Executive) exists.
- [x] 3 Mob archetypes spawn and render as billboard sprites.
- [x] Grunt AI cycles Patrol → Chase → Attack with raycaster LOS.
- [x] Lieutenant boss fight on Floor 3: waves, cigar weakpoint, flooded arena.
- [x] Enemies drop faction-appropriate loot (ammo, health, key items).
- [x] Player weapons (Sprint 3) damage and kill enemies.

---

## Playtest notes

| Floor | Content |
|-------|---------|
| 1 | Patrolling Thug |
| 2 | Thug + Zed + Heavy |
| 3 | Lieutenant + flood/cigar/waves; elevator needs boss dead + vault |

Watch HUD for **CIGAR LIT — SHOOT** during the boss. Stay out of the basin when flood % climbs.
