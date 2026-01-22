use bevy::prelude::*;

mod audio;
mod camera;
mod controls;
mod custom_asset_types;
mod dialog_system;
mod global;
mod lighting;
mod map;
mod physics;
mod player;
mod third_party;
mod tick;
mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        global::plugin,
        third_party::plugin,
        lighting::plugin,
        audio::plugin,
        camera::plugin,
        map::plugin,
        physics::plugin,
        custom_asset_types::plugin,
        dialog_system::plugin,
        ui::plugin,
        tick::plugin,
        controls::plugin,
        player::plugin,
    ));
}
