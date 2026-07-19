//! Grid-based crate pushing (TODO-022).

use bevy::prelude::*;

use adrenochrome_engine::{Billboard, MapGrid};

use crate::floor_loader::FloorEntity;
use crate::player::{Player, PlayerMotor, PLAYER_RADIUS};

use crate::enemy::TEX_CRATE;

/// Solid-occupying pushable crate on the map grid.
#[derive(Component, Debug, Clone, Copy)]
pub struct PushableCrate {
    pub cell: IVec2,
}

pub fn spawn_crate(commands: &mut Commands, map: &mut MapGrid, pos: Vec2, scale: f32) {
    let cell = IVec2::new(pos.x.floor() as i32, pos.y.floor() as i32);
    occupy_cell(map, cell, true);
    let center = Vec2::new(cell.x as f32 + 0.5, cell.y as f32 + 0.5);
    commands.spawn((
        FloorEntity,
        Name::new("Crate"),
        PushableCrate { cell },
        Billboard::new(center, TEX_CRATE, scale),
        Transform::from_xyz(center.x, center.y, 0.0),
    ));
}

fn occupy_cell(map: &mut MapGrid, cell: IVec2, solid: bool) {
    if cell.x < 0 || cell.y < 0 {
        return;
    }
    let x = cell.x as usize;
    let y = cell.y as usize;
    if x >= map.width || y >= map.height {
        return;
    }
    // Use panel wall tex (2) when solid; clear when vacated.
    map.set(x, y, if solid { 2 } else { 0 });
}

/// If the player is jammed against a crate while wishing to move into it, push one cell.
pub fn push_crates(
    mut map: ResMut<MapGrid>,
    player: Query<&PlayerMotor, With<Player>>,
    mut crates: Query<(&mut PushableCrate, &mut Billboard, &mut Transform)>,
) {
    let Ok(motor) = player.single() else {
        return;
    };
    // Infer push intent from velocity / facing into adjacent crate.
    let wish = motor.velocity;
    if wish.length_squared() < 0.25 {
        return;
    }
    let dir = wish.normalize();
    let axis = if dir.x.abs() > dir.y.abs() {
        IVec2::new(dir.x.signum() as i32, 0)
    } else {
        IVec2::new(0, dir.y.signum() as i32)
    };
    if axis == IVec2::ZERO {
        return;
    }

    let player_cell = IVec2::new(motor.pos.x.floor() as i32, motor.pos.y.floor() as i32);
    let target_cell = player_cell + axis;

    // Snapshot crate cells for occupancy checks.
    let occupied: Vec<IVec2> = crates.iter().map(|(c, _, _)| c.cell).collect();

    for (mut crate_comp, mut billboard, mut transform) in &mut crates {
        if crate_comp.cell != target_cell {
            continue;
        }
        // Must be close enough to the crate face.
        let crate_center = Vec2::new(
            crate_comp.cell.x as f32 + 0.5,
            crate_comp.cell.y as f32 + 0.5,
        );
        if motor.pos.distance(crate_center) > 1.0 + PLAYER_RADIUS {
            continue;
        }

        let dest = crate_comp.cell + axis;
        let blocked_map = map.is_solid(dest.x as isize, dest.y as isize);
        let blocked_crate = occupied.iter().any(|c| *c == dest);
        if blocked_map || blocked_crate {
            continue;
        }

        occupy_cell(&mut map, crate_comp.cell, false);
        occupy_cell(&mut map, dest, true);
        crate_comp.cell = dest;
        let center = Vec2::new(dest.x as f32 + 0.5, dest.y as f32 + 0.5);
        billboard.pos = center;
        transform.translation.x = center.x;
        transform.translation.y = center.y;
        info!("Pushed crate to {},{}", dest.x, dest.y);
        break;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupy_and_clear_cell() {
        let mut map = MapGrid::from_rows(&["###", "#.#", "###"]);
        let cell = IVec2::new(1, 1);
        assert!(!map.is_solid(1, 1));
        occupy_cell(&mut map, cell, true);
        assert!(map.is_solid(1, 1));
        occupy_cell(&mut map, cell, false);
        assert!(!map.is_solid(1, 1));
    }
}
