# Sprint 6: Late-Game Factions & Mini-Games (Scientist Tier)

**Owner:** Gameplay + Content Teams  
**Estimate:** 2-4 weeks  
**Status:** Complete  
**Dependencies:** Sprint 4 (Enemy bundle, faction enum), Sprint 5 (hazard system, FactionRegistry), Sprint 3 (Adrenochrome Injector)

---

## Goal

Build the third faction tier: the Mad Scientist's Research forces. Introduce the
full puzzle DSL parser (RON conditions + effects), biometric doors, the Scientist
boss fight (Floor 10) with a DNA sequencer mini-game, and integrate the
Adrenochrome Injector as a counter to serum effects.

---

## Tasks

- [x] **[TODO-024]** Mad Scientist faction: Male/Female Researchers, Mutated Aides, Serum Zombies.
  - **Branch:** `sprint-06`
  - **Notes:** Researchers flee + radio; Aides erratic chase; Serum Zombies apply serum debuff. Sprites 33–42.

- [x] **[MASTER]** **[TODO-025]** Scientist boss (Floor 10): DNA sequencer, teleport + serum attacks.
  - **Branch:** `sprint-06`
  - **Notes:** HP thresholds pause for DNA (`ACGT` / `TGCA`). Teleports around arena. Serum ranged apply. Elevator needs `scientist_down && surface_open`.

- [x] **[TODO-026]** Puzzle DSL parser (conditions + effects). Biometric doors (limb collection).
  - **Branch:** `sprint-06`
  - **Notes:** Counters + `==` / `>=` comparisons; `PuzzleEffectId` effects; `Limb` pickups; biometric door `collected_limb >= 3` on Floor 8.

- [x] **[TODO-027]** Adrenochrome Injector counters serum effects.
  - **Branch:** `sprint-06`
  - **Notes:** `SerumEffect` darkens vision + drains HP; Injector cures via `cure_serum` (still costs health).

---

## Acceptance Criteria

- [x] 3+ Research archetypes spawn with variant sprites and serum attacks.
- [x] Serum debuff: darkened vision + health drain; Injector cures it.
- [x] Puzzle DSL parses RON conditions + effects; biometric doors work with limb collection.
- [x] Scientist boss on Floor 10: DNA sequencer mini-game, teleport, serum attacks.
- [x] Adrenochrome Injector counters serum effects from zombies and boss.

---

## Playtest notes

| Floor | Highlights |
|-------|------------|
| 8 | Fleeing researchers; pick up 3 limbs → biometric door |
| 9 | Serum zombies — use Injector (4) when SERUM! shows |
| 10 | Scientist — enter DNA when prompted (A/C/G/T or 1–4) |
