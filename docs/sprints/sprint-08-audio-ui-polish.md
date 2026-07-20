# Sprint 8: Audio, UI & Polish

**Owner:** Art & Audio + Systems Teams  
**Estimate:** 2-4 weeks  
**Status:** Complete  
**Dependencies:** Sprint 1 (render target for post-processing), Sprint 3 (player health/ammo for HUD), Sprint 2 (floor palettes)

---

## Goal

Build the full audio system (tracker music, footsteps, PA voice lines), the pixel
font UI (health/ammo HUD + egui terminal fallback), and the complete
post-processing stack (dither, CRT effects, palette swaps, pain flashes).

---

## Tasks

- [x] **[TODO-032]** MIDI-style tracker music, floor-specific footsteps (wet → clean), distorted PA voice lines.
  - `gameplay/src/audio/` — cluster music beds, combat intensifier, wet/industrial/clean footsteps, PA stings.
  - Placeholder WAVs in `assets/audio/` (regenerate via `scripts/gen_audio.py`).

- [x] **[TODO-033]** Pixel font UI (health/ammo) + egui fallback for terminals. Hand overlay system.
  - `engine/src/pixel_hud.rs` — 3×5 bitmap HUD blitted into the 320×200 framebuffer (CRT-treated).
  - Hand viewmodel remains raycaster-composited (HUD layer of the low-res buffer).
  - `bevy_egui` DNA sequencer + `OpenTerminal` panels at window resolution.

- [x] **[TODO-034]** Full post-processing stack: dither, CRT effects, palette swaps, pain flashes.
  - CRT shader: scanlines, barrel, CA, vignette, Bayer dither, grain, phosphor glow.
  - Pain / serum driven via `CrtMaterial::post_fx` uniforms (replaces fullscreen UI flashes).

---

## Acceptance Criteria

- [x] Tracker music plays per floor cluster, intensifies in combat.
- [x] Footstep SFX change by floor type (wet → clean).
- [x] PA voice lines trigger on floor events / boss phases.
- [x] Pixel font HUD shows health, armor, ammo, weapon, floor.
- [x] egui terminal panels open on terminal interaction.
- [x] Hand overlay renders as HUD layer (low-res buffer).
- [x] Post-processing: dither, scanlines, curvature, palette swap, pain flash all active.
