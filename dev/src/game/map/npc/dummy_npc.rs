use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSlice;

const IDENTIFIER: &str = "DummyNpc";

#[derive(Component)]
struct DummyNpc;

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
        DummyNpc
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (super::despawn_npc::<DummyNpc>, super::spawn_npc::<DummyNpc>),
    );
}
