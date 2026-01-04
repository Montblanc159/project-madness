use bevy::prelude::*;

mod ambient_audio;
mod player_audio;

pub fn plugin(app: &mut App) {
    app.add_plugins((player_audio::plugin, ambient_audio::plugin));
}
