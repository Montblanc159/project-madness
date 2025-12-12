use bevy::prelude::*;

pub mod dialogs;

pub const DEFAULT_FONT_SIZE: f32 = 45.;
pub const DEFAULT_PADDING: u8 = 25;

pub fn plugin(app: &mut App) {
    app.add_plugins(dialogs::plugin);
}
