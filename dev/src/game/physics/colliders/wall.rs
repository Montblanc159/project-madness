use bevy::prelude::*;

use crate::game::map::int_grid_objects::Wall;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, super::add_collider::<Wall>);
}
