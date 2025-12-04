use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_ldtk::{GridCoords, LevelEvent};

mod portals;

#[derive(Default, Resource)]
struct Zones<T: Component> {
    locations: HashMap<GridCoords, T>,
}

impl<T: Component> Zones<T> {
    pub fn activated(&self, grid_coords: &GridCoords) -> bool {
        self.locations.contains_key(grid_coords)
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins(portals::plugin);
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
