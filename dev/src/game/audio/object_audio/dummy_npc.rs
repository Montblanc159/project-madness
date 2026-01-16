use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::map::npc::dummy_npc::DummyNpc;

impl super::SpatialAudioParameters for DummyNpc {
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
            super::setup_spatial_object_audio::<DummyNpc>,
            super::queue_object_audio::<DummyNpc>,
        ),
    );
}
