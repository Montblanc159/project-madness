use bevy::prelude::*;

mod ambient_audio;
pub mod music;
pub mod object_audio;
mod player_audio;

/// All audio related plugins
pub fn plugin(app: &mut App) {
    app.add_plugins((
        player_audio::plugin,
        ambient_audio::plugin,
        object_audio::plugin,
        music::plugin,
    ));
}
