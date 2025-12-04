use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSlice;
use bevy_ecs_ldtk::GridCoords;

use crate::game::player::Activate;

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
        (
            super::despawn_npc::<DummyNpc>,
            super::spawn_npc::<DummyNpc>,
            super::wander::<DummyNpc>,
            super::update_npc_position::<DummyNpc>,
            activate,
        ),
    );
}

fn activate(mut activate_msg: MessageReader<Activate>, npcs: Query<&GridCoords, With<DummyNpc>>) {
    for msg in activate_msg.read() {
        for grid_coords in npcs {
            if msg.grid_coords == (*grid_coords).into() {
                println!("Stop touching me !");
            }
        }
    }
}
