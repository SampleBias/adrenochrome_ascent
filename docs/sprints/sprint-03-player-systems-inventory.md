# Sprint 3: Player Systems & Inventory

**Owner:** Gameplay Team  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** Sprint 1 (raycaster + controller), Sprint 2 (floor loader, interactables)

---

## Goal

Build the player's body: hand sprites, health/armor/inventory components, the
weapon system (pistol, shotgun, plasma rifle, Adrenochrome Injector), and combat
feedback. This sprint introduces combat to what was previously a stealth-only
scaffold.

---

## Tasks

- [ ] **[TODO-011]** Pixel-perfect hand sprite system (idle, interact glow, weapon fire animations). Integrate existing glowing hand assets. (4 days)
  - **Branch:** `todo-011`
  - **Assignee:** Dev (art integration)
  - **Dependencies:** TODO-003 (billboard sprite support in raycaster)
  - **Notes:** The raycaster (TODO-003) supports billboard sprites — the hand is a special billboard fixed to the camera (weapon viewmodel). States: idle, interact glow (when looking at an interactable), fire animation (per weapon). The master doc references "existing glowing hand assets" but the current `assets/` dir only has `fonts/FiraSans-Bold.ttf` — art assets need to be sourced/created (flag for TODO-042 asset pipeline). Use placeholder sprites until art is ready.

- [ ] **[TODO-012]** Player ECS components: `Health`, `Armor`, `Inventory` (limited slots), pain flash post-process effect. (4 days)
  - **Branch:** `todo-012`
  - **Assignee:** Dev
  - **Dependencies:** TODO-005 (state machine), TODO-002 (render target for pain flash overlay)
  - **Notes:** `Health` (0-100), `Armor` (0-100, absorbs damage), `Inventory` (limited slots — keycards, key items, weapons, ammo). Pain flash: red overlay on the render target when taking damage, fading out. The existing `src/game/constants.rs` has no health/combat constants — add them here. These components attach to the `Player` entity from `src/player/controller.rs`.

- [ ] **[TODO-013]** Weapon system: Pistol start (9mm, scarce ammo), shotgun, plasma rifle, Adrenochrome Injector (temp vision + health drain). (1 week)
  - **Branch:** `todo-013`
  - **Assignee:** Dev (strong gameplay systems)
  - **Dependencies:** TODO-011 (hand sprites), TODO-012 (inventory to hold weapons/ammo), TODO-003 (raycast hitscan)
  - **Notes:** 4 weapons:
    - **Pistol** — starting weapon, 9mm, scarce ammo, hitscan via raycaster.
    - **Shotgun** — spread hitscan, close range, found mid-game.
    - **Plasma rifle** — projectile or rapid hitscan, late game.
    - **Adrenochrome Injector** — signature weapon/item: temporary enhanced vision (reveal hidden entities/interactables) at the cost of draining health. This ties into TODO-027 (counter to serum effects in Sprint 6).
  - Each weapon: damage, fire rate, ammo type, hand sprite, fire animation, muzzle flash. Ammo scarcity is a core design pillar — tune in TODO-037.

- [ ] **[TODO-014]** Combat feedback: screen shake, muzzle flash on raycast hitscan, hit reactions. (3 days)
  - **Branch:** `todo-014`
  - **Assignee:** Dev
  - **Dependencies:** TODO-013 (weapon system), TODO-002 (render target for flash overlay)
  - **Notes:** Screen shake (camera yaw/pitch jitter on fire), muzzle flash (bright sprite at the hand position for a few frames), hit reactions (enemy sprite flash + knockback on hit). Hitscan: the raycaster's DDA ray can be reused for weapon raycasts — cast from player position along view direction, first wall or enemy hit takes damage.

---

## Parallelization

```
TODO-012 ──┐
TODO-011 ──┼──> TODO-013 ──> TODO-014
           └──> (pain flash needs TODO-002)
```

- TODO-011 and TODO-012 can run in parallel (hand sprites vs health/inventory components).
- TODO-013 depends on both (weapons need hand sprites + inventory).
- TODO-014 depends on TODO-013 (needs weapons to produce feedback).

---

## Acceptance Criteria

- [ ] Hand sprite renders in view with idle, glow, and fire states.
- [ ] Player has Health, Armor, Inventory components; damage reduces health, armor absorbs.
- [ ] Pain flash overlay triggers on damage taken.
- [ ] All 4 weapons function: pistol, shotgun, plasma rifle, Adrenochrome Injector.
- [ ] Adrenochrome Injector grants temp vision and drains health.
- [ ] Hitscan combat works via raycaster ray (enemies take damage, die).
- [ ] Screen shake + muzzle flash + hit reactions fire on combat events.
