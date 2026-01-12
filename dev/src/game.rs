use bevy::prelude::*;

mod audio;
mod camera;
mod controls;
mod custom_asset_types;
mod dialog_system;
mod map;
mod player;
mod third_party;
mod tick;
mod ui;

// Game
// ================================================================

pub fn plugin(app: &mut App) {
    app.add_plugins((
        third_party::plugin,
        audio::plugin,
        camera::plugin,
        custom_asset_types::plugin,
        dialog_system::plugin,
        ui::plugin,
        tick::plugin,
        controls::plugin,
        map::plugin,
        player::plugin,
    ));
}
