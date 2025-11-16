use bevy::prelude::*;

mod camera;
mod map;
mod player;
mod third_party;

// Game
// ================================================================

pub fn plugin(app: &mut App) {
    app.add_plugins((
        third_party::plugin,
        camera::plugin,
        map::plugin,
        player::plugin,
    ));
}
