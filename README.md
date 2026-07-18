# Adrenochrome Ascent

A first-person lo-fi horror game built with [Bevy](https://bevyengine.org).
Renders at **320×200** with a CRT upscale (scanlines, vignette, dither, palette grades).

Style references live in [`assets/images/style_reference/`](assets/images/style_reference/) —
blood-red halls, toxic green lobbies, liminal teal corridors, neon player hand.

## Premise

You wake up chained to a bed in the basement of an experimental laboratory.
Solve escape-room puzzles across floors, avoid detection by scientists
and security systems, and ascend to the surface.

Press **Space** in the current demo to cycle floor palettes (Red → Green → Teal → Black).

## Levels

| # | Name             | Core Mechanic                                  |
|---|------------------|------------------------------------------------|
| 1 | Awakening        | Break free from restraints (intro tutorial)    |
| 2 | The Corridor     | Sneak past a patrolling scientist              |
| 3 | Storage Room     | Find keycards & solve a combination lock       |
| 4 | Laboratory       | Chemical mixing puzzle, evade cameras          |
| 5 | Server Room      | Hack terminals via a grid/circuit puzzle       |
| 6 | Incinerator      | Timing puzzle — destroy evidence               |
| 7 | The Surface      | Final escape under full alarm evasion          |

## Controls

- **WASD** — Move
- **Mouse** — Look
- **E** — Interact
- **Shift** — Sprint
- **C** — Crouch
- **Esc** — Pause

## Run

```sh
cargo run
```

## Architecture

```
src/
  main.rs           # App entry, plugin wiring
  game/
    states.rs       # GameState enum (menus + 7 levels)
    constants.rs    # Tunable gameplay constants
    plugin.rs       # Core game plugin
  player/
    controller.rs   # First-person movement & look
    plugin.rs
  level/
    definitions.rs  # Static data for all 7 levels
    loader.rs       # Spawn/despawn level geometry
    plugin.rs
  puzzle/
    components.rs   # Puzzle markers (lock, keypad, lever...)
    systems.rs      # Interaction & solve logic
    plugin.rs
  enemy/
    components.rs   # Scientist, camera, detection meter
    systems.rs      # Patrol & detection AI
    plugin.rs
  ui/
    hud.rs          # Crosshair, detection meter, subtitles
    menus.rs        # Main menu, pause, game over, victory
    plugin.rs
  audio/
    plugin.rs       # Placeholder audio plugin
```

## Status

This is the **framework scaffold**. Systems are stubbed with clear TODOs
for content authoring per level.
