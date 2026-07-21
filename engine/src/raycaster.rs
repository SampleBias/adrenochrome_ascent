//! Doom-style software DDA raycaster.
//!
//! Each frame, casts one ray per screen column into [`MapGrid`], draws textured
//! walls with distance shading, paints a simple floor/ceiling, then projects
//! [`Billboard`] sprites and the [`HandOverlay`] into the 320×200 CPU
//! framebuffer sampled by the CRT upscale material.

use bevy::prelude::*;

use crate::{
    attract::paint_attract_if_needed,
    billboard::{Billboard, HandOverlay},
    frame_source::FrameSource,
    map::MapGrid,
    palette::{RENDER_HEIGHT, RENDER_WIDTH},
    pixel_hud::{draw_pixel_hud, PixelHud},
    ray_camera::RayCamera,
    render_target::LowResTarget,
    textures::{TextureSet, TEX_SIZE},
};

const CEILING: [u8; 3] = [14, 18, 16];
const FLOOR_A: [u8; 3] = [34, 26, 34];
const FLOOR_B: [u8; 3] = [18, 14, 18];
const FOG_RGB: [u8; 3] = [12, 6, 8];
const FOG_START: f32 = 3.0;
const FOG_END: f32 = 16.0;

/// Per-column wall depth for sprite occlusion (map units).
#[derive(Resource, Clone)]
pub struct DepthBuffer {
    pub z: Vec<f32>,
}

impl Default for DepthBuffer {
    fn default() -> Self {
        Self {
            z: vec![f32::INFINITY; RENDER_WIDTH as usize],
        }
    }
}

/// Cast walls / floor / ceiling / sprites into the low-res framebuffer.
pub fn render_frame(
    camera: Res<RayCamera>,
    map: Res<MapGrid>,
    textures: Res<TextureSet>,
    target: Res<LowResTarget>,
    frame_source: Res<FrameSource>,
    mut images: ResMut<Assets<Image>>,
    mut depth: ResMut<DepthBuffer>,
    billboards: Query<&Billboard>,
    hands: Query<&HandOverlay>,
    hud: Res<PixelHud>,
    time: Res<Time>,
    mut attract_tick: Local<f32>,
) {
    // Title/menus: light attract path at ~8 FPS (cuts CPU→GPU upload pressure).
    if matches!(*frame_source, FrameSource::AttractTitle) {
        let t = time.elapsed_secs();
        if frame_source.is_changed() || t - *attract_tick >= 0.125 {
            *attract_tick = t;
            paint_attract_if_needed(&target, &mut images, t);
        }
        return;
    }

    let Some(mut image) = images.get_mut(&target.0) else {
        return;
    };
    let Some(buf) = image.data.as_mut() else {
        return;
    };

    let w = RENDER_WIDTH as usize;
    let h = RENDER_HEIGHT as usize;
    depth.z.fill(f32::INFINITY);

    // --- Floor + ceiling (solid with checkered near-floor suggestion) ---
    for y in 0..h {
        let is_ceil = y < h / 2;
        for x in 0..w {
            let color = if is_ceil {
                CEILING
            } else {
                // Perspective-ish checker using screen y as fake depth.
                let row = (h - 1 - y) as f32;
                let checker = ((x / 12) + (row as usize / 8)) % 2 == 0;
                let mut c = if checker { FLOOR_A } else { FLOOR_B };
                // Blood wash near bottom.
                if y > h * 3 / 4 && checker {
                    c = [90, 24, 48];
                }
                // Darken with "distance" (higher on screen = farther floor).
                let dist_factor = (y as f32 / h as f32).clamp(0.25, 1.0);
                c = [
                    (c[0] as f32 * dist_factor) as u8,
                    (c[1] as f32 * dist_factor) as u8,
                    (c[2] as f32 * dist_factor) as u8,
                ];
                c
            };
            put_bgra(buf, w, x, y, color[0], color[1], color[2], 255);
        }
    }

    // --- Walls (DDA per column) ---
    for col in 0..w {
        let cam_x = 2.0 * col as f32 / w as f32 - 1.0;
        let ray_dir = camera.dir + camera.plane * cam_x;

        let (dist, wall_tex, wall_x, side) = cast_ray(&map, camera.pos, ray_dir);
        depth.z[col] = dist;

        // Perpendicular wall distance already from cast; line height.
        let line_h = if dist > 0.0001 {
            (h as f32 / dist) as i32
        } else {
            h as i32
        };
        let mut draw_start = -line_h / 2 + h as i32 / 2;
        let mut draw_end = line_h / 2 + h as i32 / 2;
        if draw_start < 0 {
            draw_start = 0;
        }
        if draw_end >= h as i32 {
            draw_end = h as i32 - 1;
        }

        let tex_x = ((wall_x * TEX_SIZE as f32) as usize).min(TEX_SIZE - 1);

        for y in draw_start..=draw_end {
            let d = y * 256 - h as i32 * 128 + line_h * 128;
            let tex_y = (((d * TEX_SIZE as i32) / line_h) / 256)
                .clamp(0, TEX_SIZE as i32 - 1) as usize;
            let mut px = textures.wall(wall_tex, tex_x, tex_y);
            // Side shading (y-sides darker) for depth cue.
            if side == 1 {
                px[0] = px[0] / 2;
                px[1] = px[1] / 2;
                px[2] = px[2] / 2;
            }
            let shaded = apply_fog(px, dist);
            put_bgra(
                buf,
                w,
                col,
                y as usize,
                shaded[0],
                shaded[1],
                shaded[2],
                255,
            );
        }
    }

    // --- World billboards (far→near for overdraw; cull behind/off-screen in draw) ---
    let mut sprites: Vec<&Billboard> = billboards.iter().collect();
    // Skip sprites clearly behind the camera before sort (TODO-039).
    sprites.retain(|s| {
        let rel = s.pos - camera.pos;
        rel.dot(camera.dir) > -0.5
    });
    sprites.sort_by(|a, b| {
        let da = (a.pos - camera.pos).length_squared();
        let db = (b.pos - camera.pos).length_squared();
        db.partial_cmp(&da).unwrap_or(std::cmp::Ordering::Equal)
    });

    for sprite in sprites {
        draw_billboard(
            buf,
            w,
            h,
            &depth.z,
            &camera,
            &textures,
            sprite,
        );
    }

    // --- Hand overlay (HUD layer, after world — TODO-011 / TODO-033) ---
    for hand in &hands {
        draw_hand(buf, w, h, &textures, hand, time.elapsed_secs());
    }

    // --- Pixel vitals HUD (same 320×200 buffer → CRT) ---
    draw_pixel_hud(buf, w, h, &hud);
}

/// DDA cast. Returns (perp_distance, texture_id, wall_x 0..1, side 0=x 1=y).
pub fn cast_ray(map: &MapGrid, pos: Vec2, ray_dir: Vec2) -> (f32, u8, f32, i32) {
    let mut map_x = pos.x.floor() as i32;
    let mut map_y = pos.y.floor() as i32;

    let delta_dist_x = if ray_dir.x.abs() < 1e-8 {
        1e30
    } else {
        (1.0 / ray_dir.x).abs()
    };
    let delta_dist_y = if ray_dir.y.abs() < 1e-8 {
        1e30
    } else {
        (1.0 / ray_dir.y).abs()
    };

    let (step_x, mut side_dist_x) = if ray_dir.x < 0.0 {
        (-1, (pos.x - map_x as f32) * delta_dist_x)
    } else {
        (1, (map_x as f32 + 1.0 - pos.x) * delta_dist_x)
    };
    let (step_y, mut side_dist_y) = if ray_dir.y < 0.0 {
        (-1, (pos.y - map_y as f32) * delta_dist_y)
    } else {
        (1, (map_y as f32 + 1.0 - pos.y) * delta_dist_y)
    };

    let mut side = 0;
    let mut hit = 0u8;
    for _ in 0..64 {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }
        hit = map.get(map_x as isize, map_y as isize);
        if hit != 0 {
            break;
        }
    }

    let perp = if side == 0 {
        (map_x as f32 - pos.x + (1 - step_x) as f32 / 2.0) / ray_dir.x
    } else {
        (map_y as f32 - pos.y + (1 - step_y) as f32 / 2.0) / ray_dir.y
    }
    .abs()
    .max(0.0001);

    let wall_x = if side == 0 {
        pos.y + perp * ray_dir.y
    } else {
        pos.x + perp * ray_dir.x
    };
    let wall_x = wall_x - wall_x.floor();

    (perp, hit, wall_x, side)
}

fn draw_billboard(
    buf: &mut [u8],
    w: usize,
    h: usize,
    zbuf: &[f32],
    camera: &RayCamera,
    textures: &TextureSet,
    sprite: &Billboard,
) {
    let rel = sprite.pos - camera.pos;
    // Inverse camera matrix.
    let inv_det = 1.0 / (camera.plane.x * camera.dir.y - camera.dir.x * camera.plane.y);
    let transform_x = inv_det * (camera.dir.y * rel.x - camera.dir.x * rel.y);
    let transform_y = inv_det * (-camera.plane.y * rel.x + camera.plane.x * rel.y);

    if transform_y <= 0.05 {
        return; // behind camera
    }
    // Distance / frustum cull (TODO-039).
    if transform_y > 18.0 {
        return;
    }
    let sprite_screen_x = ((w as f32 / 2.0) * (1.0 + transform_x / transform_y)) as i32;
    let sprite_h = (h as f32 / transform_y * sprite.scale).abs() as i32;
    let sprite_w = sprite_h; // square sprites

    let draw_start_y = (-sprite_h / 2 + h as i32 / 2).max(0);
    let draw_end_y = (sprite_h / 2 + h as i32 / 2).min(h as i32 - 1);
    let draw_start_x = (-sprite_w / 2 + sprite_screen_x).max(0);
    let draw_end_x = (sprite_w / 2 + sprite_screen_x).min(w as i32 - 1);
    if draw_start_x > draw_end_x || draw_start_y > draw_end_y {
        return; // fully off-screen
    }

    for stripe in draw_start_x..=draw_end_x {
        let tex_x = ((256 * (stripe - (-sprite_w / 2 + sprite_screen_x)) * TEX_SIZE as i32)
            / sprite_w)
            / 256;
        if tex_x < 0 || tex_x >= TEX_SIZE as i32 {
            continue;
        }
        if transform_y >= zbuf[stripe as usize] {
            continue; // occluded by wall
        }
        for y in draw_start_y..=draw_end_y {
            let d = y * 256 - h as i32 * 128 + sprite_h * 128;
            let tex_y = (((d * TEX_SIZE as i32) / sprite_h) / 256)
                .clamp(0, TEX_SIZE as i32 - 1) as usize;
            let mut px = textures.sprite(sprite.texture_id, tex_x as usize, tex_y);
            if px[3] < 16 {
                continue;
            }
            if sprite.flash > 0.01 {
                let f = sprite.flash.clamp(0.0, 1.0);
                px[0] = ((px[0] as f32) * (1.0 - f) + 255.0 * f).min(255.0) as u8;
                px[1] = ((px[1] as f32) * (1.0 - f) + 180.0 * f).min(255.0) as u8;
                px[2] = ((px[2] as f32) * (1.0 - f) + 180.0 * f).min(255.0) as u8;
            }
            let shaded = apply_fog(px, transform_y);
            // Alpha blend over destination.
            let di = (y as usize * w + stripe as usize) * 4;
            let dest_b = buf[di];
            let dest_g = buf[di + 1];
            let dest_r = buf[di + 2];
            let a = px[3] as f32 / 255.0;
            let r = (shaded[0] as f32 * a + dest_r as f32 * (1.0 - a)) as u8;
            let g = (shaded[1] as f32 * a + dest_g as f32 * (1.0 - a)) as u8;
            let b = (shaded[2] as f32 * a + dest_b as f32 * (1.0 - a)) as u8;
            put_bgra(buf, w, stripe as usize, y as usize, r, g, b, 255);
        }
    }
}

fn draw_hand(
    buf: &mut [u8],
    w: usize,
    h: usize,
    textures: &TextureSet,
    hand: &HandOverlay,
    time: f32,
) {
    let size = ((h as f32 * 0.42) * hand.scale) as i32;
    let cx = (hand.anchor.x * w as f32) as i32;
    let cy = (hand.anchor.y * h as f32) as i32;
    let x0 = cx - size / 2;
    let y0 = cy - size / 2;

    let pulse = if hand.glow_pulse > 0.0 {
        0.85 + 0.15 * (time * hand.glow_pulse).sin()
    } else {
        1.0
    };

    for sy in 0..size {
        for sx in 0..size {
            let x = x0 + sx;
            let y = y0 + sy;
            if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
                continue;
            }
            let tx = (sx * TEX_SIZE as i32 / size).clamp(0, TEX_SIZE as i32 - 1) as usize;
            let ty = (sy * TEX_SIZE as i32 / size).clamp(0, TEX_SIZE as i32 - 1) as usize;
            let mut px = textures.sprite(hand.texture_id, tx, ty);
            if px[3] < 16 {
                continue;
            }
            // Boost magenta / energy outline glow.
            if px[0] > 180 && px[2] > 120 {
                px[0] = ((px[0] as f32) * pulse).min(255.0) as u8;
                px[2] = ((px[2] as f32) * pulse).min(255.0) as u8;
            }
            put_bgra(buf, w, x as usize, y as usize, px[0], px[1], px[2], 255);
        }
    }

    // Muzzle flash burst near the top of the viewmodel.
    if hand.muzzle > 0.01 {
        let flash_r = (size as f32 * 0.18 * hand.muzzle) as i32;
        let fx = cx - size / 5;
        let fy = cy - size / 3;
        for dy in -flash_r..=flash_r {
            for dx in -flash_r..=flash_r {
                if dx * dx + dy * dy > flash_r * flash_r {
                    continue;
                }
                let x = fx + dx;
                let y = fy + dy;
                if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 {
                    continue;
                }
                let t = 1.0 - ((dx * dx + dy * dy) as f32) / (flash_r * flash_r) as f32;
                let r = (255.0 * t * hand.muzzle) as u8;
                let g = (200.0 * t * hand.muzzle) as u8;
                let b = (80.0 * t * hand.muzzle) as u8;
                put_bgra(buf, w, x as usize, y as usize, r, g, b, 255);
            }
        }
    }
}

fn apply_fog(px: [u8; 4], dist: f32) -> [u8; 4] {
    let t = ((dist - FOG_START) / (FOG_END - FOG_START)).clamp(0.0, 1.0);
    [
        lerp_u8(px[0], FOG_RGB[0], t),
        lerp_u8(px[1], FOG_RGB[1], t),
        lerp_u8(px[2], FOG_RGB[2], t),
        px[3],
    ]
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 * (1.0 - t) + b as f32 * t) as u8
}

#[inline]
fn put_bgra(buf: &mut [u8], w: usize, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8) {
    let i = (y * w + x) * 4;
    if i + 3 < buf.len() {
        buf[i] = b;
        buf[i + 1] = g;
        buf[i + 2] = r;
        buf[i + 3] = a;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::MapGrid;

    #[test]
    fn cast_hits_wall() {
        let map = MapGrid::from_rows(&["###", "#.#", "###"]);
        let (dist, tex, _, _) = cast_ray(&map, Vec2::new(1.5, 1.5), Vec2::new(1.0, 0.0));
        assert!(dist > 0.0 && dist < 2.0);
        assert_ne!(tex, 0);
    }
}
