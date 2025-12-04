use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSlice;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LevelEvent};

use crate::game::map::{GRID_SIZE, colliders::Collider};

mod dummy_npc;

const NPC_Z_DEPTH: f32 = 2.;

pub fn plugin(app: &mut App) {
    app.add_plugins(dummy_npc::plugin);
}

trait Npc {
    fn identifier() -> String;
    fn aseslice(server: &Res<AssetServer>) -> AseSlice;
    fn new() -> impl Bundle;
}

fn spawn_npc<T: Component + Npc>(
    mut commands: Commands,
    new_entity_instances: Query<&EntityInstance, Added<EntityInstance>>,
    server: Res<AssetServer>,
) {
    for entity_instance in new_entity_instances.iter() {
        if entity_instance.identifier == T::identifier() {
            commands.spawn((
                T::aseslice(&server),
                Sprite::default(),
                GridCoords {
                    ..entity_instance.grid.into()
                },
                Collider,
                Transform {
                    translation: bevy_ecs_ldtk::utils::grid_coords_to_translation(
                        entity_instance.grid.into(),
                        IVec2::splat(GRID_SIZE),
                    )
                    .extend(NPC_Z_DEPTH),
                    ..Default::default()
                },
                T::new(),
            ));
        }
    }
}

fn despawn_npc<T: Component + Npc>(
    npcs: Query<Entity, With<T>>,
    mut commands: Commands,
    mut level_messages: MessageReader<LevelEvent>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Despawned(_) = level_event {
            for entity in npcs {
                commands.entity(entity).despawn();
            }
        }
    }
}
