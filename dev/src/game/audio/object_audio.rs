use std::collections::HashMap;

use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, SpatialAudioEmitter, SpatialRadius};

#[derive(Resource)]
pub struct SpatialAudioChannel;

#[derive(Component)]
pub struct SpatialAudioObject;

pub trait SpatialAudioParameters {
    fn file_path(audio_id: String) -> Option<String> {
        Self::file_paths().get(&audio_id).cloned()
    }

    fn file_paths() -> HashMap<String, String>;
}

#[derive(Message)]
pub struct PlayObjectAudio {
    pub entity: Entity,
    pub audio_id: String,
}

pub fn plugin(app: &mut App) {
    app.add_audio_channel::<SpatialAudioChannel>();
    app.add_message::<PlayObjectAudio>();
    // app.add_systems(Update, play_ambient);
}

pub fn setup_spatial_object_audio<T: Component + SpatialAudioParameters>(
    mut commands: Commands,
    spatial_audio_channel: Res<AudioChannel<SpatialAudioChannel>>,
    asset_server: Res<AssetServer>,
    spatial_objects: Query<Entity, (With<SpatialAudioObject>, Added<T>)>,
) {
    for entity in spatial_objects {
        let mut audio_instances = vec![];

        if let Some(file_path) = T::file_path("default".into()) {
            audio_instances.push(
                spatial_audio_channel
                    .play(asset_server.load(file_path))
                    .looped()
                    .handle(),
            );
        }

        commands.entity(entity).insert((
            SpatialAudioEmitter {
                instances: audio_instances,
            },
            SpatialRadius { radius: 150.0 },
        ));
    }
}

pub fn queue_object_audio<T: Component + SpatialAudioParameters>(
    mut events: MessageReader<PlayObjectAudio>,
    spatial_audio_channel: Res<AudioChannel<SpatialAudioChannel>>,
    asset_server: Res<AssetServer>,
    mut spatial_objects: Query<&mut SpatialAudioEmitter, (With<SpatialAudioObject>, With<T>)>,
) {
    for event in events.read() {
        if let Ok(mut audio_emitter) = spatial_objects.get_mut(event.entity)
            && let Some(file_path) = T::file_path(event.audio_id.clone())
        {
            let audio = spatial_audio_channel
                .play(asset_server.load(file_path))
                .handle();

            audio_emitter.instances.push(audio);
        }
    }
}
