use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSlice;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LevelEvent};

use crate::game::map::GRID_SIZE;

const OBJECT_Z_DEPTH: f32 = 1.5;

// pub fn plugin(app: &mut App) {}

pub trait MapObject {
    fn identifier() -> String;
    fn aseslice(server: &Res<AssetServer>) -> AseSlice;
    fn new() -> impl Bundle;
}

pub fn spawn_object<T: Component + MapObject>(
    mut commands: Commands,
    new_entity_instances: Query<&EntityInstance, Added<EntityInstance>>,
    server: Res<AssetServer>,
) {
    for entity_instance in new_entity_instances {
        if entity_instance.identifier == T::identifier() {
            commands.spawn((
                T::aseslice(&server),
                Sprite::default(),
                GridCoords {
                    ..entity_instance.grid.into()
                },
                Transform {
                    translation: bevy_ecs_ldtk::utils::grid_coords_to_translation(
                        entity_instance.grid.into(),
                        IVec2::splat(GRID_SIZE),
                    )
                    .extend(OBJECT_Z_DEPTH),
                    ..Default::default()
                },
                T::new(),
            ));
        }
    }
}

pub fn despawn_object<T: Component + MapObject>(
    objects: Query<Entity, With<T>>,
    mut commands: Commands,
    mut level_messages: MessageReader<LevelEvent>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Despawned(_) = level_event {
            for entity in objects {
                commands.entity(entity).despawn();
            }
        }
    }
}
