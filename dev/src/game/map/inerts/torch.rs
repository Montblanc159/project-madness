use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSlice;

use crate::game::global::despawn_entity_on_level_change;

const IDENTIFIER: &str = "Torch";

#[derive(Component)]
pub struct Torch;

impl super::MapObject for Torch {
    fn identifier() -> String {
        IDENTIFIER.into()
    }
    fn aseslice(server: &Res<AssetServer>) -> AseSlice {
        AseSlice {
            name: "main".into(),
            aseprite: server.load("textures/objects/torch.aseprite"),
        }
    }
    fn new() -> impl Bundle {
        Self
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            despawn_entity_on_level_change::<Torch>,
            super::spawn_object::<Torch>,
        ),
    );
}
