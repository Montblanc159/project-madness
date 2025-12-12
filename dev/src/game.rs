use bevy::prelude::*;

mod camera;
mod controls;
mod map;
mod player;
mod third_party;
mod tick;
mod ui;

// Game
// ================================================================

pub fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        ui::plugin,
        tick::plugin,
        controls::plugin,
        third_party::plugin,
        map::plugin,
        player::plugin,
    ));
}
