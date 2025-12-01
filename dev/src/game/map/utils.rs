use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, ldtk::FieldValue};

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

pub struct Results {
    pub strings: HashMap<String, String>,
    pub floats: HashMap<String, f32>,
    pub integers: HashMap<String, i32>,
    pub bools: HashMap<String, bool>,
}

pub fn get_fields(entity_instance: &EntityInstance, identifiers: Vec<&str>) -> Results {
    let mut fields_iter = entity_instance.field_instances.iter();

    let mut results = Results {
        strings: HashMap::new(),
        floats: HashMap::new(),
        integers: HashMap::new(),
        bools: HashMap::new(),
    };

    for identifier in identifiers {
        if let Some(raw_field) = fields_iter.find(|&field| field.identifier == identifier) {
            match &raw_field.value {
                FieldValue::Int(value) => {
                    if let Some(value) = value {
                        results.integers.insert(identifier.into(), *value);
                    }
                }
                FieldValue::Float(value) => {
                    if let Some(value) = value {
                        results.floats.insert(identifier.into(), *value);
                    }
                }
                FieldValue::Bool(value) => {
                    results.bools.insert(identifier.into(), *value);
                }
                FieldValue::String(value) => {
                    if let Some(value) = value {
                        results.strings.insert(identifier.into(), value.into());
                    }
                }
                _ => (),
            };
        };
    }

    results
}
