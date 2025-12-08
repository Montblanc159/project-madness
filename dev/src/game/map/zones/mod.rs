use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LevelEvent};

use crate::game::map::{GRID_SIZE, utils};

mod portals;
pub mod wander_zones;

#[derive(Default, Resource)]
pub struct Zones<T: Component> {
    pub locations: HashMap<GridCoords, T>,
}

trait Zone {
    fn identifier() -> String;
    fn new(entity_instance: &EntityInstance) -> impl Bundle;
}

impl<T: Component> Zones<T> {
    pub fn activated(&self, grid_coords: &GridCoords) -> bool {
        self.locations.contains_key(grid_coords)
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins((portals::plugin, wander_zones::plugin));
}

fn empty_zones_cache<T: Component>(
    mut level_zones: ResMut<Zones<T>>,
    mut level_messages: MessageReader<LevelEvent>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(_) = level_event {
            level_zones.locations = HashMap::new();
        }
    }
}

fn remove_zones<T: Component>(
    zones: Query<Entity, With<T>>,
    mut commands: Commands,
    mut level_messages: MessageReader<LevelEvent>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Despawned(_) = level_event {
            for entity in zones {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn spawn_zones<T: Component + Zone>(
    mut commands: Commands,
    new_entity_instances: Query<(&EntityInstance, &Transform), Added<EntityInstance>>,
) {
    for (entity_instance, transform) in new_entity_instances.iter() {
        if &entity_instance.identifier == &T::identifier() {
            let full_span_grid_coords = utils::full_span_grid_coords(
                entity_instance.width,
                entity_instance.height,
                transform.translation,
                GRID_SIZE,
            );

            for grid_coords in full_span_grid_coords {
                let translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
                    grid_coords,
                    IVec2::splat(GRID_SIZE),
                )
                .extend(transform.translation.z);

                commands.spawn((
                    Transform {
                        scale: Vec3 {
                            x: 1.,
                            y: 1.,
                            z: 1.,
                        },
                        translation: translation,
                        ..*transform
                    },
                    T::new(entity_instance),
                    grid_coords,
                ));
            }
        }
    }
}

fn cache_zones<T: Component + Clone>(
    mut level_zones: ResMut<Zones<T>>,
    zones: Query<(&T, &GridCoords), Added<T>>,
) {
    for (zone, grid_coords) in zones {
        level_zones
            .locations
            .insert(grid_coords.clone(), zone.clone());
    }
}
