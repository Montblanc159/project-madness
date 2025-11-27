use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;

pub fn entity_top_left_pixel_position(
    translation: Vec3,
    width: i32,
    height: i32,
    grid_size: i32,
) -> Vec2 {
    let correction = if width % 2 == 0 {
        grid_size / 2
    } else {
        grid_size
    };

    Vec2 {
        x: if width > grid_size {
            translation.x + ((width / 2) - correction) as f32
        } else {
            translation.x
        },
        y: if height > grid_size {
            translation.y + ((height / 2) - correction) as f32
        } else {
            translation.y
        },
    }
}

pub fn grid_coords_from_entity_size(
    origin: GridCoords,
    width: i32,
    height: i32,
    grid_size: i32,
) -> Vec<GridCoords> {
    let rows = height / grid_size;
    let cols = width / grid_size;

    let mut grid_coords: Vec<GridCoords> = vec![];

    for row in 0..(rows) {
        let mut grid_coord = origin - GridCoords { x: 0, y: row };

        for col in 0..(cols) {
            grid_coord -= GridCoords { x: col, y: 0 };

            grid_coords.push(grid_coord);
        }
    }

    grid_coords
}
