#!/usr/bin/env python3
"""Generate placeholder lo-fi WAV assets for Sprint 8 (TODO-032)."""

from __future__ import annotations

import math
import struct
import wave
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1] / "assets" / "audio"
RATE = 22050


def write_wav(path: Path, samples: list[float], rate: int = RATE) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with wave.open(str(path), "w") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(rate)
        frames = bytearray()
        for s in samples:
            v = max(-1.0, min(1.0, s))
            frames += struct.pack("<h", int(v * 32000))
        w.writeframes(frames)
    print(f"wrote {path} ({len(samples)/rate:.2f}s)")


def tone(freq: float, dur: float, vol: float = 0.25, decay: bool = True) -> list[float]:
    n = int(RATE * dur)
    out = []
    for i in range(n):
        t = i / RATE
        env = (1.0 - t / dur) if decay else 1.0
        # square-ish lo-fi
        sq = 1.0 if math.sin(2 * math.pi * freq * t) >= 0 else -1.0
        out.append(sq * vol * env * 0.55 + math.sin(2 * math.pi * freq * t) * vol * env * 0.45)
    return out


def noise_burst(dur: float, vol: float = 0.2) -> list[float]:
    n = int(RATE * dur)
    out = []
    state = 1
    for i in range(n):
        # xorshift-ish
        state ^= (state << 13) & 0xFFFFFFFF
        state ^= (state >> 17) & 0xFFFFFFFF
        state ^= (state << 5) & 0xFFFFFFFF
        r = ((state & 0xFFFF) / 32768.0) - 1.0
        env = 1.0 - (i / n)
        out.append(r * vol * env)
    return out


def drone(freqs: list[float], dur: float, vol: float = 0.12) -> list[float]:
    n = int(RATE * dur)
    out = [0.0] * n
    for freq in freqs:
        for i in range(n):
            t = i / RATE
            # slow LFO
            lfo = 0.85 + 0.15 * math.sin(2 * math.pi * 0.15 * t)
            out[i] += math.sin(2 * math.pi * freq * t) * vol * lfo / len(freqs)
            # soft distortion
            out[i] += 0.15 * math.sin(2 * math.pi * freq * 2.0 * t) * vol / len(freqs)
    return out


def main() -> None:
    # Cluster tracker-style beds (loopable ~4s)
    write_wav(ROOT / "music_human.wav", drone([55.0, 82.5, 110.0], 4.0, 0.14))
    write_wav(ROOT / "music_hybrid.wav", drone([49.0, 73.5, 98.0, 147.0], 4.0, 0.13))
    write_wav(ROOT / "music_surface.wav", drone([41.0, 61.5, 123.0], 4.0, 0.12))

    # Combat intensity layer
    write_wav(
        ROOT / "music_combat.wav",
        tone(220.0, 0.08, 0.2)
        + [0.0] * int(RATE * 0.12)
        + tone(165.0, 0.08, 0.18)
        + [0.0] * int(RATE * 0.12)
        + tone(110.0, 0.1, 0.16)
        + [0.0] * int(RATE * 0.2),
    )

    # Footsteps
    write_wav(ROOT / "foot_wet.wav", noise_burst(0.09, 0.28) + tone(90.0, 0.05, 0.1))
    write_wav(ROOT / "foot_industrial.wav", tone(140.0, 0.04, 0.22) + noise_burst(0.05, 0.15))
    write_wav(ROOT / "foot_clean.wav", tone(190.0, 0.035, 0.18) + tone(95.0, 0.03, 0.08))

    # Distorted PA
    pa = []
    for f in (440.0, 392.0, 349.0, 330.0):
        pa += tone(f, 0.18, 0.22, decay=True)
        pa += [0.0] * int(RATE * 0.05)
    # grit
    for i, s in enumerate(pa):
        pa[i] = max(-1.0, min(1.0, s * 1.4 + (0.05 if i % 7 == 0 else 0.0)))
    write_wav(ROOT / "pa_line.wav", pa)

    # Ambient bed (generic)
    write_wav(ROOT / "ambient_hum.wav", drone([60.0, 90.0], 3.0, 0.08))

    print("done")


if __name__ == "__main__":
    main()
