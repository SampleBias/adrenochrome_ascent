# Sprint 8: Audio, UI & Polish

**Owner:** Art & Audio + Systems Teams  
**Estimate:** 2-4 weeks  
**Status:** Not started  
**Dependencies:** Sprint 1 (render target for post-processing), Sprint 3 (player health/ammo for HUD), Sprint 2 (floor palettes)

---

## Goal

Build the full audio system (tracker music, footsteps, PA voice lines), the pixel
font UI (health/ammo HUD + egui terminal fallback), and the complete
post-processing stack (dither, CRT effects, palette swaps, pain flashes). This
sprint replaces the current stub UI and empty audio plugin.

---

## Tasks

- [ ] **[TODO-032]** MIDI-style tracker music, floor-specific footsteps (wet → clean), distorted PA voice lines. (1 week)
  - **Branch:** `todo-032`
  - **Assignee:** Dev (audio)
  - **Dependencies:** TODO-007 (floor loader for floor-specific audio), TODO-002 (audio playback via bevy_audio)
  - **Notes:** The existing `src/audio/plugin.rs` is an empty stub (`// No systems yet`). Build it out:
    - **Tracker music** — MIDI-style background music per floor cluster (Human/Hybrid/Surface). Use `bevy_audio` or a tracker library. Music intensifies during combat / boss fights.
    - **Footsteps** — floor-specific footstep SFX: wet (flooded cells blocks, floors 1-3), industrial (Hybrid, floors 4-7), clean (Surface, floors 8-10). Triggered by player movement.
    - **PA voice lines** — distorted intercom announcements (boss teases, alarms, narrative). Triggered by floor events / boss phases.
  - Flag audio asset creation for TODO-042.

- [ ] **[TODO-033]** Pixel font UI (health/ammo) + egui fallback for terminals. Hand overlay system. (5 days)
  - **Branch:** `todo-033`
  - **Assignee:** Dev (UI)
  - **Dependencies:** TODO-012 (health/armor for HUD), TODO-013 (ammo for HUD), TODO-011 (hand overlay)
  - **Notes:** The existing `src/ui/hud.rs` has a crosshair, detection bar, level name, subtitle, and interaction prompt using `TextBundle` with `FiraSans-Bold.ttf`. Replace with:
    - **Pixel font UI** — health bar, armor bar, ammo counter, current weapon, floor indicator. Use a pixel/retro font (not FiraSans). Render at the 320×200 internal resolution so it's pixel-perfect with the CRT upscale.
    - **egui fallback for terminals** — when the player interacts with a terminal (TODO-009), an egui panel opens for the puzzle/terminal interface (text input, DNA sequencer, etc.). egui renders at full window resolution, not the 320×200 target.
    - **Hand overlay** — the hand sprite (TODO-011) renders as part of the HUD layer, not the world raycaster.
  - The existing `spawn_hud`/`despawn_hud`/`update_hud` system pattern is reusable — adapt the content.

- [ ] **[TODO-034]** Full post-processing stack: dither, CRT effects, palette swaps, pain flashes. (4 days)
  - **Branch:** `todo-034`
  - **Assignee:** Dev (strong Rust / shaders)
  - **Dependencies:** TODO-002 (basic CRT shader pipeline to extend)
  - **Notes:** Extend the basic CRT shader from TODO-002 into the full stack:
    - **Dithering** — ordered or Bayer dithering to simulate color depth reduction at 320×200.
    - **CRT effects** — scanlines, slight curvature, vignette, phosphor glow, chromatic aberration.
    - **Palette swaps** — per-floor palette (red → green → teal → black) applied as a color LUT or palette uniform in the shader.
    - **Pain flashes** — red overlay on damage (from TODO-012), integrated into the post-process shader rather than a separate overlay sprite.
  - All effects run on the fullscreen quad shader that upscales the 320×200 render target (TODO-002).

---

## Parallelization

```
TODO-002 ──┬──> TODO-034 (extends CRT pipeline)
TODO-012 ──┤
TODO-013 ──┼──> TODO-033 (HUD needs health/ammo)
TODO-011 ──┘
TODO-007 ──┬──> TODO-032 (audio needs floor loader)
TODO-002 ──┘
```

- TODO-032 (audio) depends on TODO-007 (floor loader) and TODO-002 (audio plugin).
- TODO-033 (UI) depends on TODO-011, TODO-012, TODO-013 (hand, health, ammo).
- TODO-034 (post-processing) depends on TODO-002 (basic CRT pipeline).
- All three can run in parallel once their dependencies are met.

---

## Acceptance Criteria

- [ ] Tracker music plays per floor cluster, intensifies in combat.
- [ ] Footstep SFX change by floor type (wet → clean).
- [ ] PA voice lines trigger on floor events / boss phases.
- [ ] Pixel font HUD shows health, armor, ammo, weapon, floor.
- [ ] egui terminal panels open on terminal interaction.
- [ ] Hand overlay renders as HUD layer.
- [ ] Post-processing: dither, scanlines, curvature, palette swap, pain flash all active.
