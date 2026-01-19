use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::player::Activate;

mod dummy;

pub trait Action {
    fn activate(&self) -> ();
}

pub fn plugin(app: &mut App) {
    app.add_plugins(dummy::plugin);
}

pub fn activate<T: Component + Action>(
    mut activate_msg: MessageReader<Activate>,
    actionables: Query<(&GridCoords, &T), With<T>>,
) {
    for msg in activate_msg.read() {
        for (grid_coords, action) in actionables {
            if msg.grid_coords == (*grid_coords).into() {
                action.activate();
            }
        }
    }
}
