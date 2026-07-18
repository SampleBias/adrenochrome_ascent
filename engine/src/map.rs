//! Grid map for the software raycaster.
//!
//! Cell `0` is empty space. Non-zero values are wall texture IDs
//! (see [`crate::textures`]). The floor loader (TODO-007) will populate
//! this resource from RON floor definitions.

use bevy::prelude::*;

/// Texture id used for solid / unknown walls when an id is out of range.
pub const WALL_FALLBACK: u8 = 1;

/// 2D grid the DDA raycaster and collision queries read.
#[derive(Resource, Clone, Debug)]
pub struct MapGrid {
    pub width: usize,
    pub height: usize,
    /// Row-major cells: index = y * width + x.
    pub cells: Vec<u8>,
}

impl MapGrid {
    pub fn new(width: usize, height: usize, fill: u8) -> Self {
        Self {
            width,
            height,
            cells: vec![fill; width * height],
        }
    }

    /// Build a map from a row-major list of rows (each row a `&str` of digits/chars).
    ///
    /// Digits `1`-`9` and letters map to wall texture ids. `.` / `0` / space = empty.
    /// `#` = wall texture 1. Other chars are treated as empty.
    pub fn from_rows(rows: &[&str]) -> Self {
        let height = rows.len();
        let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);
        let mut map = Self::new(width, height, 0);
        for (y, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                map.set(x, y, parse_cell(ch));
            }
        }
        map
    }

    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    pub fn get(&self, x: isize, y: isize) -> u8 {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return WALL_FALLBACK;
        }
        self.cells[self.index(x as usize, y as usize)]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        if x < self.width && y < self.height {
            let i = self.index(x, y);
            self.cells[i] = value;
        }
    }

    #[inline]
    pub fn is_solid(&self, x: isize, y: isize) -> bool {
        self.get(x, y) != 0
    }

    /// True if a circle of `radius` centered at `pos` overlaps any solid cell.
    pub fn collides(&self, pos: Vec2, radius: f32) -> bool {
        let min_x = (pos.x - radius).floor() as isize;
        let max_x = (pos.x + radius).floor() as isize;
        let min_y = (pos.y - radius).floor() as isize;
        let max_y = (pos.y + radius).floor() as isize;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.is_solid(x, y) {
                    let closest_x = pos.x.clamp(x as f32, x as f32 + 1.0);
                    let closest_y = pos.y.clamp(y as f32, y as f32 + 1.0);
                    let dx = pos.x - closest_x;
                    let dy = pos.y - closest_y;
                    if dx * dx + dy * dy < radius * radius {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Move `pos` by `delta`, sliding along walls (axis-separated).
    pub fn try_move(&self, pos: Vec2, delta: Vec2, radius: f32) -> Vec2 {
        let mut p = pos;
        let next_x = Vec2::new(p.x + delta.x, p.y);
        if !self.collides(next_x, radius) {
            p.x = next_x.x;
        }
        let next_y = Vec2::new(p.x, p.y + delta.y);
        if !self.collides(next_y, radius) {
            p.y = next_y.y;
        }
        p
    }
}

fn parse_cell(ch: char) -> u8 {
    match ch {
        '.' | '0' | ' ' => 0,
        '#' => 1,
        '1'..='9' => ch as u8 - b'0',
        'A' | 'a' => 10,
        'B' | 'b' => 11,
        'D' | 'd' => 3, // door
        'M' | 'm' => 2, // metal / panel
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rows_and_collision() {
        let map = MapGrid::from_rows(&["###", "#.#", "###"]);
        assert_eq!(map.width, 3);
        assert_eq!(map.height, 3);
        assert!(!map.is_solid(1, 1));
        assert!(map.is_solid(0, 0));
        assert!(map.collides(Vec2::new(0.5, 0.5), 0.2));
        assert!(!map.collides(Vec2::new(1.5, 1.5), 0.2));
    }
}
