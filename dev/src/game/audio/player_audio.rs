use std::collections::HashMap;

use bevy::{asset::LoadedFolder, prelude::*};
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioSource};
use rand::prelude::*;

use crate::game::controls::{PlayerAction, PlayerInputs};

#[derive(Resource, Default)]
struct PlayerAudioFoldersCache(HashMap<PlayerAudioVariant, Handle<LoadedFolder>>);

#[derive(Resource, Default)]
struct PlayerAudiosCache(HashMap<PlayerAudioVariant, Vec<Handle<AudioSource>>>);

#[derive(Resource, Eq, PartialEq, Hash, Clone)]
enum PlayerAudioVariant {
    Activate,
    None,
}

impl PlayerAudioVariant {
    fn variants() -> Vec<Self> {
        vec![Self::Activate]
    }

    fn get_folder_path(&self) -> &'static str {
        match self {
            Self::Activate => "audios/player/activate",
            _ => "",
        }
    }

    fn from_player_action(player_action: &PlayerAction) -> Self {
        match player_action {
            PlayerAction::Activate => Self::Activate,
            _ => Self::None,
        }
    }
}

#[derive(Resource)]
struct PlayerAudioChannel;

pub fn plugin(app: &mut App) {
    app.add_audio_channel::<PlayerAudioChannel>();
    app.init_resource::<PlayerAudiosCache>();
    app.init_resource::<PlayerAudioFoldersCache>();
    app.add_systems(Startup, load_audio_folders);
    app.add_systems(Update, react_to_player_action);
    app.add_systems(Update, cache_audios);
}

fn load_audio_folders(asset_server: Res<AssetServer>, mut cache: ResMut<PlayerAudioFoldersCache>) {
    for audio_variant in PlayerAudioVariant::variants() {
        let folder = asset_server.load_folder(audio_variant.get_folder_path());

        cache.0.insert(audio_variant, folder);
    }
}

fn cache_audios(
    mut events: MessageReader<AssetEvent<LoadedFolder>>,
    loaded_folders: Res<PlayerAudioFoldersCache>,
    loaded_folders_assets: Res<Assets<LoadedFolder>>,
    mut player_audio_cache: ResMut<PlayerAudiosCache>,
) {
    for event in events.read() {
        for (player_audio_variant, loaded_folder) in &loaded_folders.0 {
            if event.is_loaded_with_dependencies(loaded_folder) {
                player_audio_cache
                    .0
                    .insert(player_audio_variant.clone(), vec![]);

                let loaded_folder = loaded_folders_assets.get(loaded_folder).unwrap();

                for handle in loaded_folder.handles.iter() {
                    let audio = handle.clone().typed::<AudioSource>();

                    if let Some(audio_cache) = player_audio_cache.0.get_mut(player_audio_variant) {
                        audio_cache.push(audio);
                    }
                }
            }
        }
    }
}

fn react_to_player_action(
    actions: Res<PlayerInputs>,
    player_channel: Res<AudioChannel<PlayerAudioChannel>>,
    player_audio_cache: Res<PlayerAudiosCache>,
) {
    let mut rng = rand::rng();

    for action in &actions.just_pressed_actions {
        let audio_action = PlayerAudioVariant::from_player_action(action);

        if let Some(audios) = player_audio_cache.0.get(&audio_action)
            && let Some(audio) = audios.choose(&mut rng)
        {
            player_channel.play(audio.to_owned());
        }
    }
}
