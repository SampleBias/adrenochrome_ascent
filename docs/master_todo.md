# Adrenochrome Ascent - Master Project TODO List

**Project Overview**  
Retro-style first-person horror-action game built in Rust + Bevy.  
Core Loop: Explore floors, solve 3-5 interconnected puzzles, survive limited combat, reach elevator, ascend. 10 floors total.  
Key Features: Custom raycaster / low-res 320x200 CRT aesthetic, puzzle DSL, enemy factions tied to bosses, moral choice endings.

**Boss & Faction Hierarchy** (All prior designs remain valid)  
- Lieutenant (Low) + Mob Thugs / Enforcers  
- Warden (Mid) + Uniformed Security / Hazard Techs  
- Mad Scientist (High) + Male/Female Researchers / Mutated Aides  
- Evil CEO (Highest) + Executive Staff / Bodyguards / Drivers  

**Team Structure for Distribution**  
- **10 Devs** (various skill levels): Assign most TODOs.  
- **2 Master Devs**: Handle complex items marked **[MASTER]** (raycaster, DSL parser, boss AI, systems integration).  
- Use feature branches named `todo-XXX`. PRs required.  
- Dependencies noted; parallel work encouraged where possible.  
- Estimate: 2-4 weeks per sprint depending on team velocity.

---

## Sprint 1: Project Bootstrap & Core Engine Foundation
**Owner:** Engine Team (Master Devs lead)

- [ ] **[TODO-001]** Initialize Bevy 0.15+ project with Cargo workspaces (engine, gameplay, content crates). Add core plugins: `bevy_ecs`, `bevy_sprite`, `bevy_asset`, `bevy_audio`. (2 days)
- [ ] **[TODO-002]** Set up 320×200 internal render target with nearest-neighbor upscale + basic CRT shader pipeline supporting palette swaps. (4 days)
- [x] **[MASTER]** **[TODO-003]** Implement custom software raycaster (Doom-style) or `bevy_voxel` hybrid. Support billboard sprites for enemies and hand. (1 week)
- [x] **[TODO-004]** Basic first-person controller (WASD + mouse look, Doom-style movement/friction). (3 days)
- [x] **[TODO-005]** Create main game state enum (`MainMenu`, `InGame`, `ElevatorTransition`, `Ending`). (2 days)

---

## Sprint 2: Map & Floor Loading System
**Owner:** Engine + Content Teams

- [x] **[TODO-006]** Define floor data structures in RON files (`assets/floors/floor_01.ron` etc.). Include layout, palette (red → green → teal → black), ambient audio cues. (5 days)
- [x] **[TODO-007]** Floor loader system using `bevy_scene` bundles + entity spawning per floor cluster (1-3 Human, 4-7 Hybrid, 8-10 Surface). Elevator transitions with visual/audio shifts. (1 week)
- [x] **[TODO-008]** Implement global `PuzzleRegistry` resource + basic condition evaluator (e.g. `has_keycard && power_restored`). (4 days)
- [x] **[TODO-009]** Basic `Interactable` component for doors, terminals, valves with raycast interaction. (3 days)
- [x] **[TODO-010]** Auto-save system on elevator rides (RON serialization, 10 slots). (3 days)

---

## Sprint 3: Player Systems & Inventory
**Owner:** Gameplay Team

- [x] **[TODO-011]** Pixel-perfect hand sprite system (idle, interact glow, weapon fire animations). Integrate existing glowing hand assets. (4 days)
- [x] **[TODO-012]** Player ECS components: `Health`, `Armor`, `Inventory` (limited slots), pain flash post-process effect. (4 days)
- [x] **[TODO-013]** Weapon system: Pistol start (9mm, scarce ammo), shotgun, plasma rifle, Adrenochrome Injector (temp vision + health drain). (1 week)
- [x] **[TODO-014]** Combat feedback: screen shake, muzzle flash on raycast hitscan, hit reactions. (3 days)

---

## Sprint 4: Enemy Factions & AI (Lieutenant Mob Tier)
**Owner:** Gameplay + Art Teams

- [x] **[TODO-015]** Base `Enemy` bundle + faction enum (Mob, Security, Research, Executive). (3 days)
- [x] **[TODO-016]** Lieutenant faction sprites & archetypes: Foot Soldier Thugs, Enforcer Heavies, Zed Prisoners. (1 week)
- [x] **[MASTER]** **[TODO-017]** Lieutenant boss fight (Floor 3): wave summoning, cigar weakpoint, flooded cell arena logic. (1 week)
- [x] **[TODO-018]** Simple ECS behavior tree / state machine for grunts (Patrol → Chase → Attack). (5 days)
- [x] **[TODO-019]** Loot drop system tied to factions (ammo, health, key items). (3 days)

---

## Sprint 5: Mid-Game Factions & Hazards (Warden Tier)
**Owner:** Gameplay + Engine Teams

- [x] **[TODO-020]** Warden faction: Riot Guards, Patrol Security, Hazard Techs (shields, turrets, radio AI). (1 week)
- [x] **[MASTER]** **[TODO-021]** Warden boss (Floor 7): mid-fight valve puzzles, flood hazards via `WardenOverrides` resource. (1 week)
- [x] **[TODO-022]** Environmental hazards: timed valves, grid-based crate/forklift pushing. (5 days)
- [x] **[TODO-023]** Faction despawn on boss defeat + global `FactionRegistry` resource. (3 days)

---

## Sprint 6: Late-Game Factions & Mini-Games (Scientist Tier)
**Owner:** Gameplay + Content Teams

- [ ] **[TODO-024]** Mad Scientist faction: Male/Female Researchers (variant sprites), Mutated Aides, Serum Zombies. (1 week)
- [ ] **[MASTER]** **[TODO-025]** Scientist boss (Floor 10): DNA sequencer mini-game (RON DSL), teleport + serum attacks. (1 week)
- [ ] **[TODO-026]** Puzzle DSL parser (RON conditions + effects). Biometric doors (limb collection). (1 week)
- [ ] **[TODO-027]** Adrenochrome Injector integration as counter to serum effects. (4 days)

---

## Sprint 7: Executive Tier, Progression & Moral Choice
**Owner:** Content + Systems Teams

- [ ] **[TODO-028]** Executive faction: Bodyguards, Admin Secretaries, Limo Drivers (stealth alarms, ending convoy). (6 days)
- [ ] **[TODO-029]** CEO narrative climax + two endings (subjects_released flag from side-puzzles). (1 week)
- [ ] **[TODO-030]** Permanent mutation perks (one every 3 floors: speed, inventory, night vision). (4 days)
- [ ] **[TODO-031]** Side-puzzle system for tracking moral choices across floors. (5 days)

---

## Sprint 8: Audio, UI & Polish
**Owner:** Art & Audio + Systems Teams

- [ ] **[TODO-032]** MIDI-style tracker music, floor-specific footsteps (wet → clean), distorted PA voice lines. (1 week)
- [ ] **[TODO-033]** Pixel font UI (health/ammo) + egui fallback for terminals. Hand overlay system. (5 days)
- [ ] **[TODO-034]** Full post-processing stack: dither, CRT effects, palette swaps, pain flashes. (4 days)

---

## Sprint 9: Content Completion & Balancing
**Owner:** Content + Gameplay Teams

- [ ] **[TODO-035]** Detailed floor-by-floor room lists + 3–5 interconnected puzzles per floor (keycards, breakers, DNA, emails). (2 weeks)
- [ ] **[TODO-036]** Boss tease scripts (PA/terminals) + enemy wave tuning per cluster. (1 week)
- [ ] **[TODO-037]** Full weapon/enemy balance pass (ammo scarcity, puzzle vs combat ratio). (1 week)

---

## Sprint 10: Testing, Optimization & Ship
**Owner:** All Teams

- [ ] **[TODO-038]** Full playtest loop: Floor 1 → ending. Identify and fix softlocks. (1–2 weeks)
- [ ] **[MASTER]** **[TODO-039]** Performance optimization: raycaster culling, sprite batching, save system. (5 days)
- [ ] **[TODO-040]** Main menu, options, credits, ending cinematics (snowy mountain road with limos). (1 week)
- [ ] **[TODO-041]** Final integration: all factions, bosses, moral endings, cross-system polish. (final week)

---

## Cross-Cutting / Global TODOs (Assign as needed)
- [ ] **[TODO-042]** Comprehensive asset pipeline (sprites in RON/aseprite format, audio import workflow).
- [ ] **[TODO-043]** Project documentation: enemy hierarchy tables, puzzle prereq reference, Bevy component cheatsheet.
- [ ] **[TODO-044]** Git workflow setup: feature branches per TODO-XXX, team code reviews, CI basics.

**Sprint Planning Notes**  
- Master Devs should tackle **[MASTER]** items and review PRs for ECS patterns.  
- Junior devs can own art integration, basic puzzles, and content RON files.  
- Track progress in this file or split into per-sprint `todo-sprintX.md` files.  
- Update this master list as items are completed or refined.

**Next Steps for Lead Dev**  
Assign TODOs to the 10 devs based on skill (e.g., rendering to stronger Rust devs). Schedule weekly syncs per sprint.  

Let's build **Adrenochrome Ascent** — the labs are waiting.
