use bevy::prelude::*;
use bevy_firefly::prelude::*;

use crate::game::{
    global::GameState,
    map::{GRID_SIZE, int_grid_objects::Wall},
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        add_occluder::<Wall>.run_if(in_state(GameState::InGame)),
    );
}

fn add_occluder<T: Component>(mut commands: Commands, entities: Query<Entity, Added<T>>) {
    for entity in entities {
        commands.entity(entity).insert(Occluder2d::rectangle(
            GRID_SIZE as f32 + 0.01,
            GRID_SIZE as f32 + 0.01,
        ));
    }
}
