use bevy::prelude::*;

pub mod dialogs;

pub fn plugin(app: &mut App) {
    app.add_plugins(dialogs::plugin);
}
