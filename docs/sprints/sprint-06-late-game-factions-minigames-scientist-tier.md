# Sprint 6: Late-Game Factions & Mini-Games (Scientist Tier)

**Owner:** Gameplay + Content Teams  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** Sprint 4 (Enemy bundle, faction enum), Sprint 5 (hazard system, FactionRegistry), Sprint 3 (Adrenochrome Injector)

---

## Goal

Build the third faction tier: the Mad Scientist's Research forces. Introduce the
full puzzle DSL parser (RON conditions + effects), biometric doors, the Scientist
boss fight (Floor 10) with a DNA sequencer mini-game, and integrate the
Adrenochrome Injector as a counter to serum effects.

---

## Tasks

- [ ] **[TODO-024]** Mad Scientist faction: Male/Female Researchers (variant sprites), Mutated Aides, Serum Zombies. (1 week)
  - **Branch:** `todo-024`
  - **Assignee:** Dev (art + gameplay)
  - **Dependencies:** TODO-015 (Enemy bundle + faction enum), TODO-018 (grunt AI to extend)
  - **Notes:** 3+ archetypes for the Research tier:
    - **Male/Female Researchers** — variant sprites (same behavior, different art). Low combat, may flee or alert. Some are non-hostile (moral choice targets?).
    - **Mutated Aides** — fast, erratic, medium health, claw attacks.
    - **Serum Zombies** — slow, high health, apply serum effect on hit (drains player vision/health unless countered by Adrenochrome Injector — TODO-027).
  - These spawn on floors 8-10 (Surface cluster). Serum effect: a debuff that darkens vision and drains health over time; the Adrenochrome Injector (TODO-013/027) counters it.

- [ ] **[MASTER]** **[TODO-025]** Scientist boss (Floor 10): DNA sequencer mini-game (RON DSL), teleport + serum attacks. (1 week)
  - **Branch:** `todo-025`
  - **Assignee:** Master Dev
  - **Dependencies:** TODO-024 (Research faction), TODO-026 (puzzle DSL for mini-game), TODO-007 (Floor 10 loaded), TODO-013 (weapons)
  - **Notes:** Boss arena on Floor 10. Mechanics:
    - **DNA sequencer mini-game** — a puzzle defined in RON via the puzzle DSL (TODO-026). Player sequences DNA strands (pattern-matching / ordering puzzle) to damage the boss or progress phases. The DSL defines conditions (correct sequence) and effects (boss takes damage, phase advance).
    - **Teleport** — boss teleports around the arena (raycaster position change + sprite swap).
    - **Serum attacks** — boss throws serum projectiles that apply the serum debuff (countered by Adrenochrome Injector).
  - Complex multi-system boss with a DSL-driven mini-game — reserved for Master Dev.

- [ ] **[TODO-026]** Puzzle DSL parser (RON conditions + effects). Biometric doors (limb collection). (1 week)
  - **Branch:** `todo-026`
  - **Assignee:** Master Dev (DSL parser is complex)
  - **Dependencies:** TODO-008 (PuzzleRegistry + basic condition evaluator to extend)
  - **Notes:** Extend the basic `PuzzleRegistry` condition evaluator (TODO-008) into a full DSL:
    - **Conditions:** `has_keycard && power_restored`, `collected_limb == 3`, `dna_sequence_correct`, etc.
    - **Effects:** `open_door("lab_3")`, `spawn_enemy(...)`, `set_flag("subjects_released")`, `damage_boss(50)`, etc.
    - **RON format:** puzzles defined in RON files with condition + effect blocks, loaded by the floor loader.
    - **Biometric doors:** special doors requiring collected body parts (limbs) — a morbid key-item puzzle. `collected_limb` counter gates the door.
  - This DSL powers the DNA sequencer mini-game (TODO-025), biometric doors, and the moral-choice side-puzzle system (TODO-031). The `KeypadPuzzle` and `CircuitPuzzle` components from the existing `src/puzzle/components.rs` can be migrated into DSL-defined puzzles.

- [ ] **[TODO-027]** Adrenochrome Injector integration as counter to serum effects. (4 days)
  - **Branch:** `todo-027`
  - **Assignee:** Dev
  - **Dependencies:** TODO-013 (Adrenochrome Injector weapon/item), TODO-024 (serum effect from Serum Zombies)
  - **Notes:** The Adrenochrome Injector (TODO-013) grants temp vision + drains health. In Sprint 6, it also **cures the serum debuff** applied by Serum Zombies and the Scientist boss. When the player is serum-affected (vision darkened, health draining), using the Injector clears the debuff but still costs health — a risk/reward trade-off. This ties the player's signature item to the late-game faction's mechanic.

---

## Parallelization

```
TODO-026 ──┬──> TODO-025 (boss, needs DSL for mini-game)
TODO-024 ──┤
TODO-013 ──┴──> TODO-027 (Injector counters serum)
```

- TODO-024, TODO-026, and TODO-027 can start in parallel (faction art vs DSL vs Injector integration).
- TODO-025 (boss) depends on TODO-024 (faction) and TODO-026 (DSL for mini-game).
- TODO-027 depends on TODO-013 (Injector from Sprint 3) and TODO-024 (serum effect).

---

## Acceptance Criteria

- [ ] 3+ Research archetypes spawn with variant sprites and serum attacks.
- [ ] Serum debuff: darkened vision + health drain; Injector cures it.
- [ ] Puzzle DSL parses RON conditions + effects; biometric doors work with limb collection.
- [ ] Scientist boss on Floor 10: DNA sequencer mini-game, teleport, serum attacks.
- [ ] Adrenochrome Injector counters serum effects from zombies and boss.
