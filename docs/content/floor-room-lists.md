# Floor Room Lists & Puzzle Graphs (Sprint 9 / TODO-035)

Authoring reference for the 10 ascent floors. Flags persist across elevator rides until main-menu reset.

## Human cluster (floors 1–3) — puzzle-heavy

| Floor | Rooms | Puzzle chain | Combat |
|------:|-------|--------------|--------|
| 1 | Med bay, storage, breaker closet, security desk, elev lobby | keycard → bay door → breaker → email terminal → elev | 1 Thug |
| 2 | Twin corridors, holding cells, breaker niche, elev | cell key → breaker → gate → PA/clearance → elev | Thug, Zed, Heavy |
| 3 | Approach hall, dossier niche, vault approach, flood arena | dossier → keycard+power → vault → LT fight → elev | Lieutenant + Thug |

**Teases:** F1–F2 PA/terminals foreshadow the Lieutenant.

## Hybrid cluster (floors 4–7)

| Floor | Rooms | Puzzle chain | Combat |
|------:|-------|--------------|--------|
| 4 | Chem bay, crate alley, override desk | lab power → crate path → chem terminal → door → elev | Patrol, Riot |
| 5 | Cold aisle, ops desk, turret niches, uplink | email → power bus → aisle door → uplink → elev | Patrol, Tech, turrets |
| 6 | Ash pit, valve catwalk, incinerator lock | ash key → timed valve → burn lock → crate/vent → elev | Riot, Patrol |
| 7 | Valve wings, flood basin arena | mercy codes → valves A/B mid-fight → Warden down → elev | Warden + Patrols |

**Teases:** F5 security log + F6 PA foreshadow the Warden.

## Surface cluster (floors 8–10) — combat-heavier

| Floor | Rooms | Puzzle chain | Combat |
|------:|-------|--------------|--------|
| 8 | Lab wings, limb caches, biometric gate | tease mail → 3 limbs → biometric door → elev | Research + Exec |
| 9 | Serum ward, uplink, penthouse door | cross-floor limbs/exec_open → uplink → pent door → elev | Serum Zombies + Exec |
| 10 | Mad lab, surface door, convoy pad | DNA → Scientist → surface door → CEO choice → elev | Full climax cast |

**Teases:** F8–F9 foreshadow Scientist / CEO. Cross-floor: `collected_limb` / `exec_open` from F8 into F9.

## Wave tuning defaults

| Cluster | max_grunts | cooldown_secs |
|---------|-----------:|--------------:|
| Human | 2 | 9.5 |
| Hybrid | 3 | 8.0 |
| Surface | 4 | 6.5 |

Override per floor via `wave_tuning: Some(WaveTuning(...))` in RON.
