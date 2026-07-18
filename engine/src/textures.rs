//! Procedural wall / sprite textures for the raycaster.
//!
//! Placeholder art until the Aseprite pipeline (TODO-042) lands. Tuned toward
//! the lo-fi horror palette in `assets/images/style_reference/`.

use bevy::prelude::*;

pub const TEX_SIZE: usize = 64;

/// Flat RGBA8 texture atlas indexed by wall/sprite id.
#[derive(Resource, Clone)]
pub struct TextureSet {
    /// Each entry is TEX_SIZE * TEX_SIZE pixels as RGBA (r,g,b,a).
    pub walls: Vec<Vec<[u8; 4]>>,
    pub sprites: Vec<Vec<[u8; 4]>>,
}

impl TextureSet {
    pub fn procedural() -> Self {
        Self {
            walls: vec![
                solid([0, 0, 0, 255]),                 // 0 unused
                gen_brick(),                          // 1 brick / blood wall
                gen_panel(),                          // 2 metal panel
                gen_door(),                           // 3 door
                gen_tiles(),                          // 4 dirty tiles
                gen_grime(),                          // 5 asylum grime
            ],
            sprites: vec![
                gen_enemy_silhouette(),               // 0 enemy
                gen_hand(),                           // 1 player hand
                gen_wisp(),                           // 2 cyan spectral wisp
                gen_item_keycard(),                   // 3 pickup placeholder
            ],
        }
    }

    #[inline]
    pub fn wall(&self, id: u8, tx: usize, ty: usize) -> [u8; 4] {
        let tex = self
            .walls
            .get(id as usize)
            .or_else(|| self.walls.get(1))
            .expect("wall textures");
        tex[ty.min(TEX_SIZE - 1) * TEX_SIZE + tx.min(TEX_SIZE - 1)]
    }

    #[inline]
    pub fn sprite(&self, id: usize, tx: usize, ty: usize) -> [u8; 4] {
        let tex = self
            .sprites
            .get(id)
            .or_else(|| self.sprites.first())
            .expect("sprite textures");
        tex[ty.min(TEX_SIZE - 1) * TEX_SIZE + tx.min(TEX_SIZE - 1)]
    }
}

fn solid(c: [u8; 4]) -> Vec<[u8; 4]> {
    vec![c; TEX_SIZE * TEX_SIZE]
}

fn put(buf: &mut [[u8; 4]], x: usize, y: usize, c: [u8; 4]) {
    if x < TEX_SIZE && y < TEX_SIZE {
        buf[y * TEX_SIZE + x] = c;
    }
}

fn gen_brick() -> Vec<[u8; 4]> {
    let mut buf = solid([48, 18, 22, 255]);
    let mortar = [28, 12, 14, 255];
    let blood = [120, 24, 28, 255];
    for y in 0..TEX_SIZE {
        for x in 0..TEX_SIZE {
            let row = y / 8;
            let offset = if row % 2 == 0 { 0 } else { 8 };
            if y % 8 == 0 || (x + offset) % 16 == 0 {
                put(&mut buf, x, y, mortar);
            } else if (x * 13 + y * 7) % 47 < 3 {
                put(&mut buf, x, y, blood);
            } else {
                let shade = 40 + ((x + y) % 12) as u8;
                put(&mut buf, x, y, [shade.saturating_add(20), shade / 2, shade / 2, 255]);
            }
        }
    }
    buf
}

fn gen_panel() -> Vec<[u8; 4]> {
    let mut buf = solid([22, 36, 28, 255]);
    for y in 0..TEX_SIZE {
        for x in 0..TEX_SIZE {
            if x < 3 || x > TEX_SIZE - 4 || y < 3 || y > TEX_SIZE - 4 {
                put(&mut buf, x, y, [10, 16, 12, 255]);
            } else if (x + y) % 9 == 0 {
                put(&mut buf, x, y, [40, 70, 48, 255]);
            } else if y > 28 && y < 36 && x > 20 && x < 44 {
                // sickly green light strip
                put(&mut buf, x, y, [80, 180, 90, 255]);
            }
        }
    }
    buf
}

fn gen_door() -> Vec<[u8; 4]> {
    let mut buf = solid([30, 14, 16, 255]);
    for y in 0..TEX_SIZE {
        for x in 0..TEX_SIZE {
            if x == TEX_SIZE / 2 || x == TEX_SIZE / 2 - 1 {
                put(&mut buf, x, y, [55, 18, 20, 255]);
            }
            if (8..28).contains(&y) && ((10..26).contains(&x) || (38..54).contains(&x)) {
                // red-glow panes
                let glow = 140 + ((x + y) % 40) as u8;
                put(&mut buf, x, y, [glow, 40, 40, 255]);
            }
            if x < 2 || x > TEX_SIZE - 3 {
                put(&mut buf, x, y, [18, 8, 10, 255]);
            }
        }
    }
    buf
}

fn gen_tiles() -> Vec<[u8; 4]> {
    let mut buf = solid([36, 34, 40, 255]);
    for y in 0..TEX_SIZE {
        for x in 0..TEX_SIZE {
            let light = ((x / 8) + (y / 8)) % 2 == 0;
            let base = if light { [70, 62, 72, 255] } else { [22, 20, 26, 255] };
            put(&mut buf, x, y, base);
            if (x * 3 + y * 5) % 61 < 2 {
                put(&mut buf, x, y, [90, 20, 30, 255]); // stain
            }
        }
    }
    buf
}

fn gen_grime() -> Vec<[u8; 4]> {
    let mut buf = solid([40, 55, 42, 255]);
    for y in 0..TEX_SIZE {
        for x in 0..TEX_SIZE {
            let drip = (x * 17 + 11) % 23;
            if y > drip + 20 {
                let g = 30 + (y % 20) as u8;
                put(&mut buf, x, y, [g, g + 10, g, 255]);
            }
            if x % 16 == 0 {
                put(&mut buf, x, y, [20, 28, 22, 255]);
            }
        }
    }
    buf
}

fn gen_enemy_silhouette() -> Vec<[u8; 4]> {
    let mut buf = solid([0, 0, 0, 0]);
    let body = [8, 8, 12, 255];
    let shirt = [200, 30, 28, 255];
    // legs
    for y in 40..60 {
        for x in 22..30 {
            put(&mut buf, x, y, body);
        }
        for x in 34..42 {
            put(&mut buf, x, y, body);
        }
    }
    // torso (red shirt accent from style refs)
    for y in 18..42 {
        for x in 20..44 {
            put(&mut buf, x, y, shirt);
        }
    }
    // head
    for y in 6..18 {
        for x in 26..38 {
            let dx = x as isize - 32;
            let dy = y as isize - 12;
            if dx * dx + dy * dy < 40 {
                put(&mut buf, x, y, body);
            }
        }
    }
    buf
}

fn gen_hand() -> Vec<[u8; 4]> {
    let mut buf = solid([0, 0, 0, 0]);
    let outline = [240, 60, 180, 255];
    let fill = [12, 6, 14, 255];
    let glow = [180, 40, 140, 255];

    // palm
    fill_rect(&mut buf, 18, 28, 50, 58, outline);
    fill_rect(&mut buf, 20, 30, 48, 56, fill);
    // fingers
    for (fx, fy, fw, fh) in [
        (16, 8, 24, 32),
        (26, 4, 34, 30),
        (36, 6, 44, 32),
        (46, 12, 54, 34),
    ] {
        fill_rect(&mut buf, fx, fy, fw, fh, outline);
        fill_rect(&mut buf, fx + 1, fy + 1, fw - 1, fh - 1, fill);
    }
    fill_rect(&mut buf, 28, 38, 42, 44, glow);
    buf
}

fn gen_wisp() -> Vec<[u8; 4]> {
    let mut buf = solid([0, 0, 0, 0]);
    for y in 0..TEX_SIZE {
        for x in 0..TEX_SIZE {
            let dx = x as f32 - 32.0;
            let dy = y as f32 - 32.0;
            let d = (dx * dx + dy * dy).sqrt();
            if d < 26.0 {
                let a = ((1.0 - d / 26.0) * 255.0) as u8;
                let pulse = ((x + y) % 7) as u8 * 8;
                put(
                    &mut buf,
                    x,
                    y,
                    [40 + pulse, 200 + pulse / 2, 230, a.max(40)],
                );
            }
        }
    }
    buf
}

fn gen_item_keycard() -> Vec<[u8; 4]> {
    let mut buf = solid([0, 0, 0, 0]);
    fill_rect(&mut buf, 16, 24, 48, 40, [200, 180, 40, 255]);
    fill_rect(&mut buf, 20, 28, 44, 36, [40, 40, 50, 255]);
    fill_rect(&mut buf, 22, 30, 30, 34, [80, 220, 120, 255]);
    buf
}

fn fill_rect(buf: &mut [[u8; 4]], x0: usize, y0: usize, x1: usize, y1: usize, c: [u8; 4]) {
    for y in y0..y1.min(TEX_SIZE) {
        for x in x0..x1.min(TEX_SIZE) {
            put(buf, x, y, c);
        }
    }
}
