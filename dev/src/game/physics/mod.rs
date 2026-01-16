use bevy::prelude::*;

pub mod colliders;

pub fn plugin(app: &mut App) {
    app.add_plugins(colliders::plugin);
}
