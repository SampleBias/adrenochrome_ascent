# Sprint 10 playtest notes (TODO-038 / TODO-041)

Automated: `cargo test --workspace` (green).

## Softlock mitigations shipped

| Area | Fix |
|------|-----|
| Player death | `watch_player_death` → GameOver; Enter loads autosave, Esc → menu |
| Elevator cheat | `KeyL` only in debug builds |
| Floor 9 uplink | Failsafe limb if `collected_limb < 1` and `!exec_open` |
| Warden valves | `warden_valve_*` timers freeze while `combat_paused` |
| ESC Options | Soft resume — does not reload/wipe the floor |

## Manual checklist (recommended before release tag)

- [ ] Floor 1 → 10 elevators, both moral endings (Released / Contained)
- [ ] Load each of 10 save slots from main menu
- [ ] Options: volume / CRT / dither / fullscreen persist for the session
- [ ] Ending outdoor road visible behind ending text
- [ ] Lieutenant / Warden / Scientist / CEO fights completable
- [ ] No obvious softlock on F7 valves or F8–F9 limb/exec path
