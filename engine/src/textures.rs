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
                gen_hand(),                           // 1 player hand (unarmed)
                gen_wisp(),                           // 2 cyan spectral wisp
                gen_item_keycard(),                   // 3 pickup placeholder
                gen_weapon_hand(WeaponArt::Pistol, false), // 4
                gen_weapon_hand(WeaponArt::Pistol, true),  // 5 fire
                gen_weapon_hand(WeaponArt::Shotgun, false), // 6
                gen_weapon_hand(WeaponArt::Shotgun, true),  // 7 fire
                gen_weapon_hand(WeaponArt::Plasma, false),  // 8
                gen_weapon_hand(WeaponArt::Plasma, true),   // 9 fire
                gen_weapon_hand(WeaponArt::Injector, false), // 10
                gen_weapon_hand(WeaponArt::Injector, true),  // 11 fire
                // Sprint 4 Mob archetypes (idle / attack)
                gen_faction_enemy(EnemyArt::Thug, false),      // 12
                gen_faction_enemy(EnemyArt::Thug, true),       // 13
                gen_faction_enemy(EnemyArt::Heavy, false),     // 14
                gen_faction_enemy(EnemyArt::Heavy, true),      // 15
                gen_faction_enemy(EnemyArt::Zed, false),       // 16
                gen_faction_enemy(EnemyArt::Zed, true),        // 17
                gen_faction_enemy(EnemyArt::Lieutenant, false), // 18
                gen_faction_enemy(EnemyArt::Lieutenant, true),  // 19 cigar flare
                gen_loot_ammo(),                               // 20
                gen_loot_medkit(),                             // 21
                // Sprint 5 Security archetypes (idle / attack)
                gen_faction_enemy(EnemyArt::RiotGuard, false),     // 22
                gen_faction_enemy(EnemyArt::RiotGuard, true),      // 23
                gen_faction_enemy(EnemyArt::PatrolSecurity, false), // 24
                gen_faction_enemy(EnemyArt::PatrolSecurity, true),  // 25
                gen_faction_enemy(EnemyArt::HazardTech, false),    // 26
                gen_faction_enemy(EnemyArt::HazardTech, true),     // 27
                gen_faction_enemy(EnemyArt::Warden, false),        // 28
                gen_faction_enemy(EnemyArt::Warden, true),         // 29
                gen_crate_sprite(),                               // 30
                gen_turret_sprite(false),                          // 31
                gen_turret_sprite(true),                           // 32 firing
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

#[derive(Clone, Copy)]
enum EnemyArt {
    Thug,
    Heavy,
    Zed,
    Lieutenant,
    RiotGuard,
    PatrolSecurity,
    HazardTech,
    Warden,
}

fn gen_faction_enemy(kind: EnemyArt, attack: bool) -> Vec<[u8; 4]> {
    let mut buf = solid([0, 0, 0, 0]);
    let (body, shirt, accent) = match kind {
        EnemyArt::Thug => ([10, 10, 14, 255], [160, 40, 36, 255], [200, 160, 40, 255]),
        EnemyArt::Heavy => ([8, 10, 18, 255], [40, 48, 70, 255], [120, 130, 150, 255]),
        EnemyArt::Zed => ([18, 28, 16, 255], [70, 110, 50, 255], [180, 220, 80, 255]),
        EnemyArt::Lieutenant => ([12, 8, 10, 255], [90, 20, 28, 255], [220, 160, 60, 255]),
        EnemyArt::RiotGuard => ([14, 16, 22, 255], [50, 70, 90, 255], [80, 200, 220, 255]),
        EnemyArt::PatrolSecurity => ([12, 14, 20, 255], [40, 90, 60, 255], [220, 200, 60, 255]),
        EnemyArt::HazardTech => ([16, 18, 14, 255], [60, 100, 40, 255], [240, 160, 40, 255]),
        EnemyArt::Warden => ([8, 12, 16, 255], [30, 50, 70, 255], [60, 220, 160, 255]),
    };

    let wide = matches!(
        kind,
        EnemyArt::Heavy | EnemyArt::Lieutenant | EnemyArt::RiotGuard | EnemyArt::Warden
    );
    let (leg_l, leg_r, torso_x0, torso_x1, head_r) = if wide {
        (18, 28, 16, 48, 48)
    } else if matches!(kind, EnemyArt::Zed) {
        (24, 32, 22, 42, 32)
    } else {
        (22, 30, 20, 44, 40)
    };

    for y in 40..60 {
        for x in leg_l..(leg_l + 8) {
            put(&mut buf, x, y, body);
        }
        for x in leg_r..(leg_r + 8) {
            put(&mut buf, x, y, body);
        }
    }
    for y in 18..42 {
        for x in torso_x0..torso_x1 {
            put(&mut buf, x, y, shirt);
        }
    }
    for y in 6..18 {
        for x in 26..38 {
            let dx = x as isize - 32;
            let dy = y as isize - 12;
            if dx * dx + dy * dy < head_r {
                put(&mut buf, x, y, body);
            }
        }
    }

    // Accent: bat / shield / claws / cigar.
    match kind {
        EnemyArt::Thug => {
            fill_rect(&mut buf, 8, 20, 18, 40, accent);
            if attack {
                fill_rect(&mut buf, 4, 14, 14, 28, accent);
            }
        }
        EnemyArt::Heavy => {
            fill_rect(&mut buf, 14, 22, 50, 38, accent);
            if attack {
                fill_rect(&mut buf, 10, 18, 22, 44, [200, 80, 60, 255]);
            }
        }
        EnemyArt::Zed => {
            fill_rect(&mut buf, 14, 28, 22, 36, accent);
            fill_rect(&mut buf, 42, 28, 50, 36, accent);
            if attack {
                fill_rect(&mut buf, 10, 24, 20, 40, accent);
                fill_rect(&mut buf, 44, 24, 54, 40, accent);
            }
        }
        EnemyArt::Lieutenant => {
            // Lit cigar tip (weakpoint cue).
            fill_rect(&mut buf, 40, 14, 50, 18, accent);
            let tip = if attack {
                [255, 220, 80, 255]
            } else {
                [255, 120, 40, 255]
            };
            fill_rect(&mut buf, 48, 12, 54, 20, tip);
        }
        EnemyArt::RiotGuard => {
            // Frontal riot shield.
            fill_rect(&mut buf, 8, 16, 22, 48, accent);
            fill_rect(&mut buf, 10, 18, 20, 46, [30, 40, 50, 255]);
            if attack {
                fill_rect(&mut buf, 6, 14, 24, 50, [120, 240, 255, 255]);
            }
        }
        EnemyArt::PatrolSecurity => {
            fill_rect(&mut buf, 44, 22, 52, 34, accent); // radio pack
            if attack {
                fill_rect(&mut buf, 46, 18, 54, 28, [255, 220, 80, 255]);
            }
        }
        EnemyArt::HazardTech => {
            fill_rect(&mut buf, 40, 30, 54, 42, accent); // toolbox
            if attack {
                fill_rect(&mut buf, 42, 20, 52, 30, [255, 180, 40, 255]);
            }
        }
        EnemyArt::Warden => {
            fill_rect(&mut buf, 18, 10, 46, 18, accent); // visor
            if attack {
                fill_rect(&mut buf, 20, 10, 44, 18, [80, 255, 180, 255]);
            }
        }
    }
    buf
}

fn gen_crate_sprite() -> Vec<[u8; 4]> {
    let mut buf = solid([0, 0, 0, 0]);
    fill_rect(&mut buf, 12, 16, 52, 56, [120, 80, 40, 255]);
    fill_rect(&mut buf, 16, 20, 48, 52, [90, 60, 30, 255]);
    fill_rect(&mut buf, 12, 34, 52, 38, [60, 40, 20, 255]);
    fill_rect(&mut buf, 30, 16, 34, 56, [60, 40, 20, 255]);
    buf
}

fn gen_turret_sprite(firing: bool) -> Vec<[u8; 4]> {
    let mut buf = solid([0, 0, 0, 0]);
    fill_rect(&mut buf, 20, 36, 44, 56, [40, 48, 56, 255]);
    fill_rect(&mut buf, 26, 18, 38, 40, [70, 80, 90, 255]);
    fill_rect(&mut buf, 28, 8, 36, 22, [100, 110, 120, 255]);
    if firing {
        fill_rect(&mut buf, 30, 2, 34, 12, [255, 200, 80, 255]);
    }
    buf
}

fn gen_loot_ammo() -> Vec<[u8; 4]> {
    let mut buf = solid([0, 0, 0, 0]);
    fill_rect(&mut buf, 18, 28, 46, 48, [180, 150, 60, 255]);
    fill_rect(&mut buf, 22, 32, 42, 44, [40, 40, 30, 255]);
    fill_rect(&mut buf, 26, 22, 30, 32, [200, 180, 80, 255]);
    fill_rect(&mut buf, 34, 22, 38, 32, [200, 180, 80, 255]);
    buf
}

fn gen_loot_medkit() -> Vec<[u8; 4]> {
    let mut buf = solid([0, 0, 0, 0]);
    fill_rect(&mut buf, 18, 24, 46, 48, [200, 40, 50, 255]);
    fill_rect(&mut buf, 22, 28, 42, 44, [240, 240, 240, 255]);
    fill_rect(&mut buf, 30, 30, 34, 42, [200, 40, 50, 255]);
    fill_rect(&mut buf, 26, 34, 38, 38, [200, 40, 50, 255]);
    buf
}

#[derive(Clone, Copy)]
enum WeaponArt {
    Pistol,
    Shotgun,
    Plasma,
    Injector,
}

/// First-person weapon viewmodel (hand + gun), fire frame brightens the muzzle.
fn gen_weapon_hand(kind: WeaponArt, firing: bool) -> Vec<[u8; 4]> {
    let mut buf = gen_hand();
    let metal = [90, 90, 100, 255];
    let dark = [30, 28, 36, 255];
    let accent = match kind {
        WeaponArt::Pistol => [180, 160, 80, 255],
        WeaponArt::Shotgun => [120, 80, 50, 255],
        WeaponArt::Plasma => [40, 200, 220, 255],
        WeaponArt::Injector => [200, 40, 90, 255],
    };

    // Grip / receiver over the palm.
    fill_rect(&mut buf, 24, 34, 40, 52, dark);
    fill_rect(&mut buf, 26, 36, 38, 50, metal);

    match kind {
        WeaponArt::Pistol => {
            fill_rect(&mut buf, 18, 22, 46, 34, dark);
            fill_rect(&mut buf, 20, 24, 44, 32, metal);
            fill_rect(&mut buf, 12, 26, 22, 30, accent);
        }
        WeaponArt::Shotgun => {
            fill_rect(&mut buf, 10, 24, 50, 36, dark);
            fill_rect(&mut buf, 12, 26, 48, 34, metal);
            fill_rect(&mut buf, 8, 28, 14, 32, accent);
            fill_rect(&mut buf, 28, 36, 36, 48, dark);
        }
        WeaponArt::Plasma => {
            fill_rect(&mut buf, 14, 20, 48, 34, dark);
            fill_rect(&mut buf, 16, 22, 46, 32, [50, 70, 90, 255]);
            fill_rect(&mut buf, 10, 24, 18, 30, accent);
            // Cell glow strip.
            fill_rect(&mut buf, 30, 26, 42, 28, accent);
        }
        WeaponArt::Injector => {
            fill_rect(&mut buf, 22, 16, 42, 36, dark);
            fill_rect(&mut buf, 24, 18, 40, 34, [60, 20, 40, 255]);
            fill_rect(&mut buf, 28, 10, 36, 20, accent);
            // Syringe tip.
            fill_rect(&mut buf, 30, 4, 34, 12, [200, 200, 210, 255]);
        }
    }

    if firing {
        let flash = match kind {
            WeaponArt::Plasma => [120, 240, 255, 255],
            WeaponArt::Injector => [255, 80, 140, 255],
            _ => [255, 220, 120, 255],
        };
        fill_rect(&mut buf, 4, 20, 16, 34, flash);
        fill_rect(&mut buf, 6, 18, 14, 36, [255, 255, 200, 255]);
    }

    buf
}

fn fill_rect(buf: &mut [[u8; 4]], x0: usize, y0: usize, x1: usize, y1: usize, c: [u8; 4]) {
    for y in y0..y1.min(TEX_SIZE) {
        for x in x0..x1.min(TEX_SIZE) {
            put(buf, x, y, c);
        }
    }
}
