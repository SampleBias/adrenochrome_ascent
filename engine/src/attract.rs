//! NES-era attract backdrop painted into the 320×200 framebuffer.
//!
//! Late-80s cartridge title vibe: limited palette, mansion silhouette, blood
//! sky, neon door — matches the lo-fi horror refs without running the raycaster.

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

/// Lit window with a dark frame and prison-style bars.
fn draw_barred_window(
    buf: &mut [u8],
    w: usize,
    wx: usize,
    wy: usize,
    ww: usize,
    wh: usize,
    glass: [u8; 3],
    frame: [u8; 3],
    bar: [u8; 3],
) {
    // Outer frame (1px border around the glass).
    for dy in 0..wh {
        for dx in 0..ww {
            let on_frame = dx == 0 || dx == ww - 1 || dy == 0 || dy == wh - 1;
            put(buf, w, wx + dx, wy + dy, if on_frame { frame } else { glass });
        }
    }
    // Vertical bars through the glass (skip the outer frame).
    let bar_xs = [ww / 3, (2 * ww) / 3];
    for &bx in &bar_xs {
        if bx == 0 || bx >= ww - 1 {
            continue;
        }
        for dy in 1..wh - 1 {
            put(buf, w, wx + bx, wy + dy, bar);
        }
    }
    // One horizontal crossbar mid-pane.
    let by = wh / 2;
    for dx in 1..ww - 1 {
        put(buf, w, wx + dx, wy + by, bar);
    }
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

    // Mansion / asylum — denser NES silhouette with eaves, chimney, barred windows.
    let base_y = h * 5 / 8;
    let mid = w / 2;
    let half_w = 78;
    let left = mid - half_w;
    let right = mid + half_w;
    let roof_h = 32;
    let roof_top = base_y - 62;
    let eave_y = roof_top + roof_h;
    let wall = [14, 10, 16];
    let wall_hi = [22, 16, 26];
    let wall_lo = [8, 6, 10];
    let roof = [28, 12, 18];
    let roof_ridge = [48, 20, 28];
    let frame = [4, 3, 6];
    let bar = [10, 8, 12];

    // Main pitched roof + body fill with light brick/stone noise.
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
                // Mortar lines + subtle vertical pilasters.
                let brick = (y - eave_y) % 3 == 0;
                let mortar_x = (x + (y / 3) * 2) % 5 == 0;
                let pilaster = (x as i32 - left as i32) % 26 == 0 || (right as i32 - x as i32) % 26 == 0;
                if brick {
                    wall_lo
                } else if mortar_x {
                    [10, 7, 12]
                } else if pilaster {
                    wall_hi
                } else if hash2(x as u32 + 3, y as u32) % 17 == 0 {
                    [18, 12, 20]
                } else {
                    wall
                }
            };
            put(buf, w, x, y, rgb);
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

    // Lit barred windows (sick green / blood red — NES limited accents).
    let windows = [
        (left + 14, eave_y + 6, [180, 40, 48]),
        (left + 36, eave_y + 6, [40, 160, 70]),
        (right - 50, eave_y + 6, [180, 40, 48]),
        (right - 28, eave_y + 6, [40, 160, 70]),
        (left + 24, eave_y + 16, [200, 50, 60]),
        (right - 40, eave_y + 16, [50, 170, 80]),
    ];
    for (wx, wy, col) in windows {
        draw_barred_window(buf, w, wx, wy, 9, 11, col, frame, bar);
    }

    // Central double door — cyan neon pulse (style-ref uncanny accent).
    let door_w = 16;
    let door_h = 24;
    let door_x = mid - door_w / 2;
    let door_y = base_y - door_h;
    let glow = (90.0 + pulse * 140.0) as u8;
    for dy in 0..door_h {
        for dx in 0..door_w {
            let edge = dx == 0 || dx == door_w - 1 || dy == 0;
            let seam = dx == door_w / 2 - 1 || dx == door_w / 2;
            let panel = (dy > 3 && dy < 11 && dx > 2 && dx < door_w / 2 - 2)
                || (dy > 3 && dy < 11 && dx > door_w / 2 + 1 && dx < door_w - 3)
                || (dy > 13 && dy < door_h - 2 && dx > 2 && dx < door_w / 2 - 2)
                || (dy > 13 && dy < door_h - 2 && dx > door_w / 2 + 1 && dx < door_w - 3);
            let rgb = if edge {
                [glow / 3, glow, glow.saturating_add(20)]
            } else if seam {
                [glow / 5, glow / 2, glow.saturating_add(8) / 2]
            } else if panel {
                [8, 14, 20]
            } else {
                [12, 20, 28]
            };
            put(buf, w, door_x + dx, door_y + dy, rgb);
        }
    }
    // Door knobs.
    put(buf, w, door_x + door_w / 2 - 3, door_y + door_h / 2, [glow / 2, glow, glow]);
    put(buf, w, door_x + door_w / 2 + 2, door_y + door_h / 2, [glow / 2, glow, glow]);

    // Ground blood wash under the door.
    for x in door_x.saturating_sub(20)..(door_x + door_w + 20).min(w) {
        for y in base_y..(base_y + 6).min(h) {
            let fade = 1.0 - (x as i32 - w as i32 / 2).unsigned_abs() as f32 / 40.0;
            if fade > 0.0 {
                put(
                    buf,
                    w,
                    x,
                    y,
                    [
                        (90.0 * fade) as u8,
                        (16.0 * fade) as u8,
                        (32.0 * fade) as u8,
                    ],
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
