use std::collections::HashMap;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSlice;
use bevy_ecs_ldtk::GridCoords;

use crate::game::{
    audio::object_audio::{
        PlayObjectAudio, SpatialAudioObject, SpatialAudioParameters, queue_object_audio,
        setup_spatial_object_audio,
    },
    dialog_system::{DialogFilePath, DialogKnot, DialogState},
    player::Activate,
};

const IDENTIFIER: &str = "DummyNpc";

#[derive(Component)]
struct DummyNpc;

#[derive(Bundle)]
struct DummyNpcBundle {
    dummy_npc: DummyNpc,
    wanderer: super::Wanderer,
    talkable: super::Talkable,
    dialog_file_path: DialogFilePath,
    dialog_state: DialogState,
    dialog_knot: DialogKnot,
    avatar_file_path: super::AvatarFilePath,
    npc_name: super::NpcName,
    spatial_audio_object: SpatialAudioObject,
}

impl super::Npc for DummyNpc {
    fn identifier() -> String {
        IDENTIFIER.to_string()
    }

    fn aseslice(server: &Res<AssetServer>) -> AseSlice {
        AseSlice {
            name: "player_idle".into(),
            aseprite: server.load("textures/npcs/dummy_npc.aseprite"),
        }
    }

    fn new() -> impl Bundle {
        DummyNpcBundle {
            dummy_npc: DummyNpc,
            wanderer: super::Wanderer,
            talkable: super::Talkable,
            dialog_file_path: DialogFilePath("dialogs/dummy_npc.ink.json".into()),
            dialog_state: DialogState("".into()),
            dialog_knot: DialogKnot("".into()),
            avatar_file_path: super::AvatarFilePath("textures/npcs/dummy_npc_avatar.png".into()),
            npc_name: super::NpcName("Dummy Npc".into()),
            spatial_audio_object: SpatialAudioObject,
        }
    }
}

impl SpatialAudioParameters for DummyNpc {
    fn file_paths() -> HashMap<String, String> {
        let mut paths = HashMap::new();
        paths.insert(
            "default".into(),
            "audios/objects/dummy_npc/default.ogg".into(),
        );

        paths.insert(
            "activate".into(),
            "audios/objects/dummy_npc/activate.ogg".into(),
        );

        paths
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            super::despawn_npc::<DummyNpc>,
            super::spawn_npc::<DummyNpc>,
            super::update_npc_position::<DummyNpc>,
            setup_spatial_object_audio::<DummyNpc>,
            queue_object_audio::<DummyNpc>,
            activate,
        ),
    );
}

fn activate(
    mut activate_msg: MessageReader<Activate>,
    mut event: MessageWriter<PlayObjectAudio>,
    npcs: Query<(Entity, &GridCoords), With<DummyNpc>>,
) {
    for msg in activate_msg.read() {
        for (entity, grid_coords) in npcs {
            if msg.grid_coords == (*grid_coords).into() {
                println!("Stop touching me !");

                event.write(PlayObjectAudio {
                    entity,
                    audio_id: "activate".into(),
                });
            }
        }
    }
}
