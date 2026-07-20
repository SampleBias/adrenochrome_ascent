# Sprint 7: Executive Tier, Progression & Moral Choice

**Owner:** Content + Systems Teams  
**Estimate:** 2-4 weeks  
**Status:** Complete  
**Dependencies:** Sprint 4-6 (all faction tiers), Sprint 2 (puzzle registry), Sprint 3 (inventory)

---

## Goal

Build the final faction tier: the CEO's Executive forces. Implement the moral
choice system that tracks player decisions across all floors, the two endings
(subjects released vs. not), permanent mutation perks (progression), and the
side-puzzle system that feeds the moral choices.

---

## Tasks

- [x] **[TODO-028]** Executive faction: Bodyguards, Admin Secretaries, Limo Drivers (stealth alarms, ending convoy).
  - Archetypes + sprites (44–51); `FloorAlarm` resource; Admin Secretaries raise floor-wide chase.
  - Spawn on floors 8–10 alongside Research; Limo Drivers + CEO on Floor 10.

- [x] **[TODO-029]** CEO narrative climax + two endings (`subjects_released` / `moral_score`).
  - Floor 10 confront/deal interactables after Scientist; `resolve_ending_from_flags` on Ending enter.

- [x] **[TODO-030]** Permanent mutation perks at floors 3 / 6 / 9 (speed, inventory slots, night vision).
  - `MutationPerks` resource; persisted in saves; night vision shares hand-glow with injector.

- [x] **[TODO-031]** Side-puzzle moral choices via DSL flags; persist; FactionRegistry spare/execute.
  - `MoralChoice` / `MoralBump` / `TriggerAlarm` in content + interact/effects.
  - Spare flags (`spare_mob`, `spare_security`, `spare_research`) arm before boss death.

---

## Acceptance Criteria

- [x] 3 Executive archetypes spawn with stealth alarm and escort behavior.
- [x] Two endings branch on `subjects_released` / moral score.
- [x] Mutation perks granted at floors 3, 6, 9 (speed, inventory, night vision).
- [x] Side-puzzles track moral choices via DSL flags, persisted in saves.
- [x] CEO narrative climax triggers the appropriate ending cinematic.
