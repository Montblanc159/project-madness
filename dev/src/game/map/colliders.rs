use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Collider;

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    collider: Collider,
}

#[derive(Default, Resource)]
pub struct LevelColliders {
    collider_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelColliders {
    pub fn in_collider(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.collider_locations.contains(grid_coords)
    }
}

pub fn plugin(app: &mut App) {
    app.register_ldtk_int_cell::<ColliderBundle>(1);
    app.init_resource::<LevelColliders>();
    app.add_systems(
        Update,
        (
            cache_level_bounds,
            cache_collider_locations,
            empty_collider_locations_cache,
        ),
    );
}

fn cache_level_bounds(
    mut level_messages: MessageReader<LevelEvent>,
    mut level_colliders: ResMut<LevelColliders>,
    ldtk_project_entities: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let ldtk_project_entities = ldtk_project_entities.into_inner();

    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets.get(ldtk_project_entities).unwrap();
            let level = ldtk_project.get_raw_level_by_iid(level_iid.get()).unwrap();

            level_colliders.level_width = level.px_wid / super::GRID_SIZE;
            level_colliders.level_height = level.px_hei / super::GRID_SIZE;
        }
    }
}

fn empty_collider_locations_cache(
    mut level_colliders: ResMut<LevelColliders>,
    mut level_messages: MessageReader<LevelEvent>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Despawned(_) = level_event {
            level_colliders.collider_locations = HashSet::new();
        }
    }
}

fn cache_collider_locations(
    mut level_colliders: ResMut<LevelColliders>,
    colliders: Query<&GridCoords, With<Collider>>,
) {
    for collider in colliders {
        level_colliders.collider_locations.insert(*collider);
    }
}
