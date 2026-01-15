use std::collections::HashMap;

use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, SpatialAudioEmitter, SpatialRadius};

/// All spatial related audio channel
#[derive(Resource)]
pub struct SpatialAudioChannel;

/// Identifies an object that has spatial audio
#[derive(Component)]
pub struct SpatialAudioObject;

/// Trait used to set spatial audio parameters for an object
pub trait SpatialAudioParameters {
    /// Specific file path fetched in a list of path
    /// identified by an audio_id
    fn file_path(audio_id: String) -> Option<String> {
        Self::file_paths().get(&audio_id).cloned()
    }

    /// Lists all audio file paths with an identifier
    /// to fetch them
    fn file_paths() -> HashMap<String, String>;
}

/// For interactive/actionable objects.
/// Instructs a system to play an audio in the list of files
/// linked to the entity
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

/// Plays and spatializes default audio to a spatial audio Component
/// If no default, does nothing (objects can have triggered spatialized sounds while
/// not emitting anything continuously)
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

/// Queues linked entity audios
/// Upon receiving `PlayObjectAudio` message
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
