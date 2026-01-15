use bevy::prelude::*;
use bevy_ecs_ldtk::LevelEvent;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl};

use crate::game::map::CurrentLevelInfos;

#[derive(Resource)]
struct AmbientAudioChannel;

pub fn plugin(app: &mut App) {
    app.add_audio_channel::<AmbientAudioChannel>();
    app.add_systems(Update, play_ambient);
}

/// Plays ambient audio using level identifier
/// and fetching dynamically corresponding file
/// in level folder.
/// Uses .ron file to add settings to audio file
fn play_ambient(
    mut level_messages: MessageReader<LevelEvent>,
    background: Res<AudioChannel<AmbientAudioChannel>>,
    asset_server: Res<AssetServer>,
    level_infos: Res<CurrentLevelInfos>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(_) = level_event {
            background.stop();
            background.play(asset_server.load(format!(
                "audios/levels/{}/ambient.ogg.ron",
                level_infos.identifier
            )));
        }
    }
}
