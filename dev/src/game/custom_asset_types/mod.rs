use bevy::prelude::*;

pub mod ink_json;

pub fn plugin(app: &mut App) {
    app.add_plugins(ink_json::plugin);
}
