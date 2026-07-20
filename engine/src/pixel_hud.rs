//! 320×200 pixel HUD blit (TODO-033) — bars + tiny bitmap font into the CRT buffer.

use bevy::prelude::*;

/// Gameplay-driven HUD state sampled by the raycaster each frame.
#[derive(Resource, Debug, Clone)]
pub struct PixelHud {
    pub enabled: bool,
    pub hp: f32,
    pub hp_max: f32,
    pub armor: f32,
    pub armor_max: f32,
    pub ammo: u32,
    pub weapon: String,
    pub floor: u8,
    pub floor_label: String,
    pub status: String,
    pub prompt: String,
    pub pa_line: String,
}

impl Default for PixelHud {
    fn default() -> Self {
        Self {
            enabled: false,
            hp: 100.0,
            hp_max: 100.0,
            armor: 0.0,
            armor_max: 100.0,
            ammo: 0,
            weapon: "PISTOL".into(),
            floor: 1,
            floor_label: String::new(),
            status: String::new(),
            prompt: String::new(),
            pa_line: String::new(),
        }
    }
}

/// Draw HUD into a BGRA8 framebuffer (`w`×`h`).
pub fn draw_pixel_hud(buf: &mut [u8], w: usize, h: usize, hud: &PixelHud) {
    if !hud.enabled || buf.len() < w * h * 4 {
        return;
    }

    // Floor label top-left
    let title = if hud.floor_label.is_empty() {
        format!("F{}", hud.floor)
    } else {
        format!("F{} {}", hud.floor, hud.floor_label)
    };
    draw_text(buf, w, h, 4, 4, &title, [200, 190, 170, 255]);

    // HP / ARM bars bottom-left
    draw_bar(buf, w, h, 4, h - 18, 72, 5, hud.hp / hud.hp_max.max(1.0), [180, 40, 48, 255]);
    draw_text(buf, w, h, 4, h - 26, "HP", [180, 70, 70, 255]);
    draw_bar(
        buf,
        w,
        h,
        4,
        h - 10,
        72,
        4,
        hud.armor / hud.armor_max.max(1.0),
        [70, 120, 180, 255],
    );

    // Weapon + ammo bottom-right
    let ammo_line = format!("{} {}", hud.weapon, hud.ammo);
    let ax = w.saturating_sub(4 + ammo_line.len() * 4);
    draw_text(buf, w, h, ax, h - 14, &ammo_line, [210, 200, 160, 255]);

    // Status / boss line
    if !hud.status.is_empty() {
        draw_text(buf, w, h, 4, 14, &hud.status, [220, 180, 80, 255]);
    }

    // Interaction prompt centered bottom
    if !hud.prompt.is_empty() {
        let px = w.saturating_sub(hud.prompt.len() * 4) / 2;
        draw_text(buf, w, h, px, h - 36, &hud.prompt, [160, 220, 180, 255]);
    }

    // PA banner top center
    if !hud.pa_line.is_empty() {
        let px = w.saturating_sub(hud.pa_line.len() * 4) / 2;
        fill_rect(buf, w, h, 0, 0, w, 12, [8, 10, 12, 200]);
        draw_text(buf, w, h, px.max(2), 3, &hud.pa_line, [180, 220, 160, 255]);
    }
}

fn draw_bar(
    buf: &mut [u8],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    fill: f32,
    color: [u8; 4],
) {
    fill_rect(buf, w, h, x, y, width, height, [20, 18, 22, 220]);
    let filled = ((width as f32 - 2.0) * fill.clamp(0.0, 1.0)) as usize;
    if filled > 0 {
        fill_rect(buf, w, h, x + 1, y + 1, filled, height.saturating_sub(2), color);
    }
}

fn fill_rect(
    buf: &mut [u8],
    w: usize,
    h: usize,
    x: usize,
    y: usize,
    rw: usize,
    rh: usize,
    color: [u8; 4],
) {
    for py in y..y.saturating_add(rh).min(h) {
        for px in x..x.saturating_add(rw).min(w) {
            put(buf, w, px, py, color);
        }
    }
}

fn put(buf: &mut [u8], w: usize, x: usize, y: usize, color: [u8; 4]) {
    let i = (y * w + x) * 4;
    if i + 3 >= buf.len() {
        return;
    }
    // BGRA
    let a = color[3] as f32 / 255.0;
    if a >= 0.99 {
        buf[i] = color[2];
        buf[i + 1] = color[1];
        buf[i + 2] = color[0];
        buf[i + 3] = 255;
    } else {
        let inv = 1.0 - a;
        buf[i] = (buf[i] as f32 * inv + color[2] as f32 * a) as u8;
        buf[i + 1] = (buf[i + 1] as f32 * inv + color[1] as f32 * a) as u8;
        buf[i + 2] = (buf[i + 2] as f32 * inv + color[0] as f32 * a) as u8;
        buf[i + 3] = 255;
    }
}

fn draw_text(buf: &mut [u8], w: usize, h: usize, x: usize, y: usize, text: &str, color: [u8; 4]) {
    let mut cx = x;
    for ch in text.chars().take(40) {
        if let Some(glyph) = glyph(ch) {
            for row in 0..5 {
                let bits = glyph[row];
                for col in 0..3 {
                    if bits & (1 << (2 - col)) != 0 {
                        put(buf, w, cx + col, y + row, color);
                    }
                }
            }
        }
        cx += 4;
        if cx + 3 >= w {
            break;
        }
        let _ = h;
    }
}

/// Tiny 3×5 glyphs (bit rows). Covers digits, A–Z, and a few punctuation.
fn glyph(ch: char) -> Option<[u8; 5]> {
    let c = ch.to_ascii_uppercase();
    Some(match c {
        ' ' => [0, 0, 0, 0, 0],
        '-' => [0, 0, 0b111, 0, 0],
        '.' => [0, 0, 0, 0, 0b010],
        '!' => [0b010, 0b010, 0b010, 0, 0b010],
        '/' => [0b001, 0b010, 0b010, 0b010, 0b100],
        ':' => [0, 0b010, 0, 0b010, 0],
        '|' => [0b010, 0b010, 0b010, 0b010, 0b010],
        '0' => [0b111, 0b101, 0b101, 0b101, 0b111],
        '1' => [0b010, 0b110, 0b010, 0b010, 0b111],
        '2' => [0b111, 0b001, 0b111, 0b100, 0b111],
        '3' => [0b111, 0b001, 0b111, 0b001, 0b111],
        '4' => [0b101, 0b101, 0b111, 0b001, 0b001],
        '5' => [0b111, 0b100, 0b111, 0b001, 0b111],
        '6' => [0b111, 0b100, 0b111, 0b101, 0b111],
        '7' => [0b111, 0b001, 0b010, 0b010, 0b010],
        '8' => [0b111, 0b101, 0b111, 0b101, 0b111],
        '9' => [0b111, 0b101, 0b111, 0b001, 0b111],
        'A' => [0b010, 0b101, 0b111, 0b101, 0b101],
        'B' => [0b110, 0b101, 0b110, 0b101, 0b110],
        'C' => [0b011, 0b100, 0b100, 0b100, 0b011],
        'D' => [0b110, 0b101, 0b101, 0b101, 0b110],
        'E' => [0b111, 0b100, 0b110, 0b100, 0b111],
        'F' => [0b111, 0b100, 0b110, 0b100, 0b100],
        'G' => [0b011, 0b100, 0b101, 0b101, 0b011],
        'H' => [0b101, 0b101, 0b111, 0b101, 0b101],
        'I' => [0b111, 0b010, 0b010, 0b010, 0b111],
        'J' => [0b001, 0b001, 0b001, 0b101, 0b010],
        'K' => [0b101, 0b101, 0b110, 0b101, 0b101],
        'L' => [0b100, 0b100, 0b100, 0b100, 0b111],
        'M' => [0b101, 0b111, 0b111, 0b101, 0b101],
        'N' => [0b101, 0b111, 0b111, 0b111, 0b101],
        'O' => [0b010, 0b101, 0b101, 0b101, 0b010],
        'P' => [0b110, 0b101, 0b110, 0b100, 0b100],
        'Q' => [0b010, 0b101, 0b101, 0b111, 0b011],
        'R' => [0b110, 0b101, 0b110, 0b101, 0b101],
        'S' => [0b011, 0b100, 0b010, 0b001, 0b110],
        'T' => [0b111, 0b010, 0b010, 0b010, 0b010],
        'U' => [0b101, 0b101, 0b101, 0b101, 0b111],
        'V' => [0b101, 0b101, 0b101, 0b101, 0b010],
        'W' => [0b101, 0b101, 0b111, 0b111, 0b101],
        'X' => [0b101, 0b101, 0b010, 0b101, 0b101],
        'Y' => [0b101, 0b101, 0b010, 0b010, 0b010],
        'Z' => [0b111, 0b001, 0b010, 0b100, 0b111],
        '[' => [0b011, 0b010, 0b010, 0b010, 0b011],
        ']' => [0b110, 0b010, 0b010, 0b010, 0b110],
        _ => return None,
    })
}
