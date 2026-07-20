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

    // Mansion / asylum block silhouette.
    let base_y = h * 5 / 8;
    let left = w / 2 - 70;
    let right = w / 2 + 70;
    let roof_top = base_y - 55;

    for y in roof_top..base_y {
        for x in left..right {
            // Pitched roof triangle.
            let mid = w / 2;
            let roof_slope = (y - roof_top) * 70 / 28;
            let in_roof = y < roof_top + 28 && x + roof_slope >= mid && x <= mid + roof_slope;
            let in_body = y >= roof_top + 28;
            if in_roof || in_body {
                put(buf, w, x, y, [8, 6, 10]);
            }
        }
    }

    // Lit windows (sick green / blood red — NES limited accents).
    let windows = [
        (left + 18, roof_top + 34, [180, 40, 48]),
        (left + 40, roof_top + 34, [40, 160, 70]),
        (right - 48, roof_top + 34, [180, 40, 48]),
        (right - 26, roof_top + 34, [40, 160, 70]),
        (left + 28, roof_top + 48, [200, 50, 60]),
        (right - 40, roof_top + 48, [50, 170, 80]),
    ];
    for (wx, wy, col) in windows {
        for dy in 0..6 {
            for dx in 0..5 {
                put(buf, w, wx + dx, wy + dy, col);
            }
        }
    }

    // Central door — cyan neon pulse (style-ref uncanny accent).
    let door_x = w / 2 - 6;
    let door_y = base_y - 22;
    let glow = (90.0 + pulse * 140.0) as u8;
    for dy in 0..20 {
        for dx in 0..12 {
            let edge = dx == 0 || dx == 11 || dy == 0;
            let rgb = if edge {
                [glow / 3, glow, glow.saturating_add(20)]
            } else {
                [12, 20, 28]
            };
            put(buf, w, door_x + dx, door_y + dy, rgb);
        }
    }

    // Ground blood wash under the door.
    for x in door_x.saturating_sub(20)..(door_x + 32).min(w) {
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
