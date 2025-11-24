use bevy::prelude::*;

mod portals;

pub fn plugin(app: &mut App) {
    app.add_plugins(portals::plugin);
}
