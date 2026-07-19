# Sprint 3: Player Systems & Inventory

**Owner:** Gameplay Team  
**Estimate:** 2-4 weeks  
**Status:** Complete  
**Dependencies:** Sprint 1 (raycaster + controller), Sprint 2 (floor loader, interactables)

---

## Goal

Build the player's body: hand sprites, health/armor/inventory components, the
weapon system (pistol, shotgun, plasma rifle, Adrenochrome Injector), and combat
feedback. This sprint introduces combat to what was previously a stealth-only
scaffold.

---

## Tasks

- [x] **[TODO-011]** Pixel-perfect hand sprite system (idle, interact glow, weapon fire animations). Integrate existing glowing hand assets. (4 days)
  - **Branch:** `sprint-03`
  - **Notes:** Procedural weapon viewmodels (sprite ids 4–11). `HandOverlay` states via `update_hand_viewmodel`: idle, interact glow, fire + muzzle.

- [x] **[TODO-012]** Player ECS components: `Health`, `Armor`, `Inventory` (limited slots), pain flash post-process effect. (4 days)
  - **Branch:** `sprint-03`
  - **Notes:** Components on `Player`; `PainFlash` resource + fullscreen UI overlay. Autosave persists health/armor/inventory.

- [x] **[TODO-013]** Weapon system: Pistol start (9mm, scarce ammo), shotgun, plasma rifle, Adrenochrome Injector (temp vision + health drain). (1 week)
  - **Branch:** `sprint-03`
  - **Notes:** Hitscan via `cast_ray` + billboard proximity. Injector grants `AdrenoVision` and drains health. Debug grants: F5/F6/F7.

- [x] **[TODO-014]** Combat feedback: screen shake, muzzle flash on raycast hitscan, hit reactions. (3 days)
  - **Branch:** `sprint-03`
  - **Notes:** Shake applied after motor sync; muzzle on `HandOverlay`; enemy billboards (tex 0) get `CombatTarget` + `HitFlash` tint.

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

- [x] Hand sprite renders in view with idle, glow, and fire states.
- [x] Player has Health, Armor, Inventory components; damage reduces health, armor absorbs.
- [x] Pain flash overlay triggers on damage taken.
- [x] All 4 weapons function: pistol, shotgun, plasma rifle, Adrenochrome Injector.
- [x] Adrenochrome Injector grants temp vision and drains health.
- [x] Hitscan combat works via raycaster ray (enemies take damage, die).
- [x] Screen shake + muzzle flash + hit reactions fire on combat events.

---

## Controls (playtest)

| Input | Action |
|-------|--------|
| LMB / Ctrl | Fire |
| 1–4 | Select weapon (if owned) |
| F5 / F6 / F7 | Grant shotgun / plasma / injector + ammo |
| E | Interact |
| L | Force elevator (debug) |
