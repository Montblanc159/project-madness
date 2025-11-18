use bevy::prelude::*;

mod camera;
mod map;
mod player;
mod third_party;
mod tick;

// Game
// ================================================================

pub fn plugin(app: &mut App) {
    app.add_plugins((
        tick::plugin,
        third_party::plugin,
        map::plugin,
        player::plugin,
        camera::plugin,
    ));
}
