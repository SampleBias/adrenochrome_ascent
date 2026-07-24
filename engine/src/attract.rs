//! NES-era attract backdrop painted into the 320×200 framebuffer.
//!
//! Late-80s cartridge title vibe: limited palette, asylum silhouette, blood
//! sky, institutional decay — dark brick clarity with grimy doors/windows.

use bevy::prelude::*;

use crate::palette::{RENDER_HEIGHT, RENDER_WIDTH};
use crate::render_target::LowResTarget;

fn put(buf: &mut [u8], w: usize, x: usize, y: usize, rgb: [u8; 3]) {
    if x >= w || y >= RENDER_HEIGHT as usize {
        return;
    }
    let i = (y * w + x) * 4;
    if i + 3 >= buf.len() {
        return;
    }
    // BGRA8
    buf[i] = rgb[2];
    buf[i + 1] = rgb[1];
    buf[i + 2] = rgb[0];
    buf[i + 3] = 255;
}

fn hash2(x: u32, y: u32) -> u32 {
    let mut n = x.wrapping_mul(374761393).wrapping_add(y.wrapping_mul(668265263));
    n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    n ^ (n >> 16)
}

fn mix(a: [u8; 3], b: [u8; 3], t: u8) -> [u8; 3] {
    // t = 0..=16 toward b
    let t = t.min(16) as u16;
    [
        ((a[0] as u16 * (16 - t) + b[0] as u16 * t) / 16) as u8,
        ((a[1] as u16 * (16 - t) + b[1] as u16 * t) / 16) as u8,
        ((a[2] as u16 * (16 - t) + b[2] as u16 * t) / 16) as u8,
    ]
}

/// Recessed institutional window: rusted metal frame, dirty glass, wire mesh,
/// moss bleed — decay theme with sharp pixel clarity.
fn draw_decay_window(
    buf: &mut [u8],
    w: usize,
    wx: usize,
    wy: usize,
    ww: usize,
    wh: usize,
    glow: [u8; 3],
    flicker: f32,
) {
    let recess = [18, 10, 12];
    let frame_outer = [42, 28, 24];
    let frame_rust = [78, 38, 28];
    let frame_dark = [22, 14, 12];
    let mesh = [28, 22, 20];
    let moss = [36, 52, 28];
    let mold = [18, 28, 16];
    let drip = [48, 32, 24];
    let spark = [120, 110, 100];

    let lit = [
        (glow[0] as f32 * (0.55 + flicker * 0.45)) as u8,
        (glow[1] as f32 * (0.55 + flicker * 0.45)) as u8,
        (glow[2] as f32 * (0.55 + flicker * 0.45)) as u8,
    ];

    // Recess / shadow lip around the opening.
    for dy in 0..wh {
        for dx in 0..ww {
            let on_lip = dx == 0 || dx == ww - 1 || dy == 0 || dy == wh - 1;
            if on_lip {
                put(buf, w, wx + dx, wy + dy, recess);
            }
        }
    }

    // Inner rusted metal frame (2px).
    for dy in 1..wh - 1 {
        for dx in 1..ww - 1 {
            let on_frame = dx <= 2 || dx >= ww - 3 || dy <= 2 || dy >= wh - 3;
            if !on_frame {
                continue;
            }
            let hsh = hash2((wx + dx) as u32, (wy + dy) as u32);
            let rgb = if hsh % 7 == 0 {
                frame_rust
            } else if hsh % 5 == 0 {
                frame_dark
            } else if dx == 1 || dy == 1 {
                frame_outer
            } else {
                frame_dark
            };
            put(buf, w, wx + dx, wy + dy, rgb);
        }
    }

    // Dirty glass pane with sickly interior light.
    for dy in 3..wh - 3 {
        for dx in 3..ww - 3 {
            let fx = wx + dx;
            let fy = wy + dy;
            let hsh = hash2(fx as u32, fy as u32);
            let mut rgb = lit;

            // Vertical water / rust streaks.
            if (fx % 3 == 0 && hsh % 3 != 0) || (hsh % 19 == 0) {
                rgb = mix(rgb, drip, 7);
            }
            // Moss / mold bloom toward bottom corners.
            let near_bottom = dy as f32 / wh as f32;
            if near_bottom > 0.55 && (hsh % 4 == 0 || dx <= 4 || dx >= ww - 5) {
                rgb = mix(rgb, moss, (6.0 + near_bottom * 8.0) as u8);
            }
            if near_bottom > 0.7 && hsh % 5 == 0 {
                rgb = mix(rgb, mold, 10);
            }
            // Grime blotches — darken glass, kill clean glow.
            if hsh % 11 == 0 {
                rgb = [
                    rgb[0].saturating_sub(28),
                    rgb[1].saturating_sub(22),
                    rgb[2].saturating_sub(18),
                ];
            }
            // Sparse highlight sparkles (brick-ref grit).
            if hsh % 47 == 0 {
                rgb = spark;
            }

            put(buf, w, fx, fy, rgb);
        }
    }

    // Wire mesh / security screen over glass.
    for dy in 3..wh - 3 {
        for dx in 3..ww - 3 {
            if (dx + dy) % 3 == 0 || dx % 4 == 0 {
                let hsh = hash2((wx + dx + 9) as u32, (wy + dy) as u32);
                if hsh % 3 != 0 {
                    put(buf, w, wx + dx, wy + dy, mesh);
                }
            }
        }
    }

    // Single thicker cross-mullion (institutional casement).
    let mx = ww / 2;
    let my = wh / 2;
    for dy in 2..wh - 2 {
        put(buf, w, wx + mx, wy + dy, frame_dark);
        if mx + 1 < ww - 2 {
            put(buf, w, wx + mx + 1, wy + dy, frame_rust);
        }
    }
    for dx in 2..ww - 2 {
        put(buf, w, wx + dx, wy + my, frame_dark);
    }

    // Bottom sill moss drip onto brick.
    for dx in 1..ww - 1 {
        let hsh = hash2((wx + dx) as u32, (wy + wh) as u32);
        if hsh % 3 == 0 {
            put(buf, w, wx + dx, wy + wh, moss);
        }
        if hsh % 5 == 0 && wy + wh + 1 < RENDER_HEIGHT as usize {
            put(buf, w, wx + dx, wy + wh + 1, mold);
        }
    }
}

/// Heavy industrial metal door: rust, viewing slot, peeling notices, grit.
fn draw_decay_door(
    buf: &mut [u8],
    w: usize,
    door_x: usize,
    door_y: usize,
    door_w: usize,
    door_h: usize,
    pulse: f32,
) {
    let metal = [38, 36, 34];
    let metal_hi = [58, 52, 46];
    let metal_lo = [22, 20, 18];
    let rust = [92, 42, 28];
    let rust_dark = [56, 26, 18];
    let frame = [28, 18, 16];
    let frame_hi = [48, 30, 24];
    let moss = [32, 48, 26];
    let notice = [70, 62, 48];
    let notice_stain = [52, 40, 28];
    let slot_glass = [
        (28.0 + pulse * 18.0) as u8,
        (48.0 + pulse * 22.0) as u8,
        (36.0 + pulse * 10.0) as u8,
    ];
    let latch = [72, 68, 58];
    let spark = [130, 118, 105];

    // Outer recessed jamb.
    for dy in 0..door_h {
        for dx in 0..door_w {
            let edge = dx == 0 || dx == door_w - 1 || dy == 0;
            if edge {
                put(
                    buf,
                    w,
                    door_x + dx,
                    door_y + dy,
                    if dy == 0 || dx == 0 { frame_hi } else { frame },
                );
            }
        }
    }

    // Door face — gunmetal with rust blooms and grit.
    for dy in 1..door_h {
        for dx in 1..door_w - 1 {
            let fx = door_x + dx;
            let fy = door_y + dy;
            let hsh = hash2(fx as u32 * 3, fy as u32 * 5);
            let mut rgb = if hsh % 13 == 0 {
                metal_hi
            } else if hsh % 9 == 0 {
                metal_lo
            } else {
                metal
            };

            // Rust along edges + random blooms.
            let edge_rust = dx <= 2 || dx >= door_w - 3 || dy >= door_h - 3;
            if edge_rust || hsh % 17 == 0 {
                rgb = mix(rgb, rust, if edge_rust { 9 } else { 5 });
            }
            if hsh % 23 == 0 {
                rgb = mix(rgb, rust_dark, 8);
            }
            // Moss / damp at the base.
            if dy > door_h * 3 / 4 && hsh % 4 == 0 {
                rgb = mix(rgb, moss, 7);
            }
            // Single-pixel sparkles for brick-ref clarity.
            if hsh % 61 == 0 {
                rgb = spark;
            }

            put(buf, w, fx, fy, rgb);
        }
    }

    // Raised panel lines (industrial plate seams).
    for dx in 3..door_w - 3 {
        put(buf, w, door_x + dx, door_y + door_h / 3, metal_lo);
        put(buf, w, door_x + dx, door_y + (door_h * 2) / 3, metal_lo);
    }
    for dy in 2..door_h - 1 {
        put(buf, w, door_x + door_w / 2, door_y + dy, metal_lo);
    }

    // Small high viewing slot — dirty wire-glass, not neon.
    let slot_w = 7;
    let slot_h = 5;
    let slot_x = door_x + (door_w - slot_w) / 2;
    let slot_y = door_y + 4;
    for dy in 0..slot_h {
        for dx in 0..slot_w {
            let on_rim = dx == 0 || dx == slot_w - 1 || dy == 0 || dy == slot_h - 1;
            let rgb = if on_rim {
                frame
            } else if (dx + dy) % 2 == 0 {
                mesh_tone(slot_glass)
            } else {
                let hsh = hash2((slot_x + dx) as u32, (slot_y + dy) as u32);
                if hsh % 4 == 0 {
                    mix(slot_glass, rust_dark, 6)
                } else {
                    slot_glass
                }
            };
            put(buf, w, slot_x + dx, slot_y + dy, rgb);
        }
    }

    // Weathered paper notices taped mid-door.
    for (ox, oy, nw, nh) in [(3usize, door_h / 2 - 1, 4usize, 5usize), (door_w - 7, door_h / 2 + 2, 3, 4)]
    {
        for dy in 0..nh {
            for dx in 0..nw {
                let hsh = hash2((door_x + ox + dx) as u32, (door_y + oy + dy) as u32);
                let rgb = if hsh % 5 == 0 { notice_stain } else { notice };
                put(buf, w, door_x + ox + dx, door_y + oy + dy, rgb);
            }
        }
    }

    // Latch / handle block.
    let hx = door_x + door_w - 5;
    let hy = door_y + door_h / 2;
    for dy in 0..3 {
        for dx in 0..3 {
            put(buf, w, hx + dx, hy + dy, if dx == 1 && dy == 1 { latch } else { metal_lo });
        }
    }
    put(buf, w, hx + 1, hy + 1, spark);
}

fn mesh_tone(base: [u8; 3]) -> [u8; 3] {
    [
        base[0].saturating_sub(14),
        base[1].saturating_sub(10),
        base[2].saturating_sub(12),
    ]
}

/// Paint the title attract scene. Called at a low rate from [`crate::raycaster::render_frame`].
pub fn draw_attract_title(buf: &mut [u8], time_secs: f32) {
    let w = RENDER_WIDTH as usize;
    let h = RENDER_HEIGHT as usize;
    let pulse = (time_secs * 2.2).sin() * 0.5 + 0.5;

    for y in 0..h {
        for x in 0..w {
            let mut rgb = if y < h * 5 / 8 {
                // Blood sky — vertical NES-like banding.
                let t = y as f32 / (h as f32 * 0.62);
                let band = ((y / 4) % 3) as u8;
                let r = (18.0 + t * 70.0) as u8;
                let g = (4.0 + t * 8.0) as u8;
                let b = (10.0 + t * 14.0) as u8;
                [r.saturating_add(band * 3), g, b.saturating_add(band)]
            } else {
                // Checkered ground (liminal hall suggestion).
                let cell = ((x / 10) + (y / 8)) % 2 == 0;
                if cell {
                    [28, 18, 32]
                } else {
                    [14, 10, 16]
                }
            };

            // Sparse starfield in the upper sky.
            if y < h / 2 {
                let hsh = hash2(x as u32, y as u32);
                if hsh % 220 == 0 {
                    let twinkle = if ((hsh >> 8) ^ (time_secs as u32).wrapping_mul(3)) % 5 == 0 {
                        255
                    } else {
                        160
                    };
                    rgb = [twinkle, twinkle, twinkle.saturating_sub(20)];
                }
            }

            put(buf, w, x, y, rgb);
        }
    }

    // Asylum facade — dark red brick clarity with institutional decay accents.
    let base_y = h * 5 / 8;
    let mid = w / 2;
    let half_w = 78;
    let left = mid - half_w;
    let right = mid + half_w;
    let roof_h = 32;
    let roof_top = base_y - 62;
    let eave_y = roof_top + roof_h;
    // Brick palette from the dark maroon reference.
    let brick_a = [72, 22, 28];
    let brick_b = [54, 16, 20];
    let brick_c = [88, 30, 34];
    let mortar = [10, 6, 8];
    let brick_spark = [140, 120, 110];
    let moss = [34, 50, 28];
    let roof = [28, 12, 18];
    let roof_ridge = [48, 20, 28];

    // Main pitched roof + body fill with running-bond brick.
    for y in roof_top..base_y {
        let roof_slope = ((y - roof_top) * half_w) / roof_h;
        for x in left..right {
            let in_roof = y < eave_y && x + roof_slope >= mid && x <= mid + roof_slope;
            let in_body = y >= eave_y;
            if !(in_roof || in_body) {
                continue;
            }
            let rgb = if in_roof {
                let on_ridge = (x as i32 - mid as i32).unsigned_abs() <= 1;
                let edge = x + roof_slope == mid || x == mid + roof_slope;
                if on_ridge {
                    roof_ridge
                } else if edge {
                    [18, 8, 12]
                } else if hash2(x as u32, y as u32) % 11 == 0 {
                    [34, 14, 20]
                } else {
                    roof
                }
            } else {
                let row = (y - eave_y) / 3;
                let offset = if row % 2 == 0 { 0 } else { 3 };
                let on_mortar_h = (y - eave_y) % 3 == 2;
                let on_mortar_v = (x + offset) % 6 == 5;
                let hsh = hash2(x as u32, y as u32);
                if on_mortar_h || on_mortar_v {
                    mortar
                } else if hsh % 53 == 0 {
                    brick_spark
                } else if hsh % 7 == 0 {
                    brick_c
                } else if hsh % 3 == 0 {
                    brick_b
                } else {
                    brick_a
                }
            };
            put(buf, w, x, y, rgb);
        }
    }

    // Moss / mold stains low on the facade (decay theme).
    for y in (eave_y + 18)..base_y {
        for x in left..right {
            let hsh = hash2(x as u32 + 11, y as u32 + 7);
            let near_ground = (y as f32 - eave_y as f32) / (base_y - eave_y) as f32;
            if near_ground > 0.55 && hsh % (4 + ((1.0 - near_ground) * 8.0) as u32) == 0 {
                put(buf, w, x, y, moss);
            }
        }
    }

    // Eave overhang strip.
    for x in (left.saturating_sub(3))..(right + 3).min(w) {
        put(buf, w, x, eave_y.saturating_sub(1), [36, 16, 22]);
        put(buf, w, x, eave_y, [20, 10, 14]);
    }

    // Chimney (left of ridge).
    let chim_x = mid - 22;
    let chim_top = roof_top + 6;
    for y in chim_top..(roof_top + 22) {
        for dx in 0..8 {
            let rgb = if dx == 0 || dx == 7 || y == chim_top {
                [40, 18, 24]
            } else if hash2((chim_x + dx) as u32, y as u32) % 5 == 0 {
                [32, 14, 18]
            } else {
                [24, 10, 14]
            };
            put(buf, w, chim_x + dx, y, rgb);
        }
    }
    // Chimney cap.
    for dx in 0..10 {
        put(buf, w, chim_x + dx - 1, chim_top.saturating_sub(1), [50, 22, 28]);
    }

    // Foundation / stoop ledge.
    for x in left..right {
        put(buf, w, x, base_y.saturating_sub(1), [6, 4, 8]);
    }
    for x in (mid - 14)..(mid + 14) {
        put(buf, w, x, base_y, [12, 8, 14]);
        if base_y + 1 < h {
            put(buf, w, x, base_y + 1, [8, 5, 10]);
        }
    }

    // Grimy institutional windows — dim sickly glow, not arcade neon.
    let windows = [
        (left + 12, eave_y + 5, [70, 42, 38]),
        (left + 34, eave_y + 5, [48, 70, 42]),
        (right - 52, eave_y + 5, [70, 42, 38]),
        (right - 30, eave_y + 5, [48, 70, 42]),
        (left + 22, eave_y + 20, [62, 38, 34]),
        (right - 42, eave_y + 20, [42, 62, 38]),
    ];
    for (i, (wx, wy, col)) in windows.iter().enumerate() {
        let flick = 0.65
            + 0.35
                * ((time_secs * (1.7 + i as f32 * 0.35) + i as f32).sin() * 0.5 + 0.5);
        draw_decay_window(buf, w, *wx, *wy, 12, 13, *col, flick);
    }

    // Heavy rusted metal door with viewing slot + taped notices.
    let door_w = 18;
    let door_h = 26;
    let door_x = mid - door_w / 2;
    let door_y = base_y - door_h;
    draw_decay_door(buf, w, door_x, door_y, door_w, door_h, pulse);

    // Damp stain / blood wash under the door.
    for x in door_x.saturating_sub(20)..(door_x + door_w + 20).min(w) {
        for y in base_y..(base_y + 6).min(h) {
            let fade = 1.0 - (x as i32 - w as i32 / 2).unsigned_abs() as f32 / 40.0;
            if fade > 0.0 {
                let hsh = hash2(x as u32, y as u32);
                let moss_mix = hsh % 3 == 0;
                put(
                    buf,
                    w,
                    x,
                    y,
                    if moss_mix {
                        [
                            (28.0 * fade) as u8,
                            (42.0 * fade) as u8,
                            (22.0 * fade) as u8,
                        ]
                    } else {
                        [
                            (72.0 * fade) as u8,
                            (18.0 * fade) as u8,
                            (24.0 * fade) as u8,
                        ]
                    },
                );
            }
        }
    }

    // Scanline darken every other row — cheap NES/CRT crunch in the buffer.
    for y in (0..h).step_by(2) {
        for x in 0..w {
            let i = (y * w + x) * 4;
            if i + 2 < buf.len() {
                buf[i] = buf[i].saturating_sub(18);
                buf[i + 1] = buf[i + 1].saturating_sub(18);
                buf[i + 2] = buf[i + 2].saturating_sub(18);
            }
        }
    }
}

/// Upload attract pixels when [`crate::frame_source::FrameSource::AttractTitle`] is active.
pub fn paint_attract_if_needed(
    target: &LowResTarget,
    images: &mut Assets<Image>,
    time_secs: f32,
) {
    let Some(mut image) = images.get_mut(&target.0) else {
        return;
    };
    let Some(buf) = image.data.as_mut() else {
        return;
    };
    draw_attract_title(buf, time_secs);
}
