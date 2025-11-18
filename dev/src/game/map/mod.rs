use std::collections::HashSet;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub const GRID_SIZE: i32 = 16;

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
    app.insert_resource(LevelSelection::index(0));
    // app.insert_resource(LdtkSettings {
    //     level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
    //         load_level_neighbors: true,
    //     },
    //     ..Default::default()
    // });
    app.register_ldtk_int_cell::<ColliderBundle>(1);
    app.init_resource::<LevelColliders>();
    app.add_systems(Startup, map_setup);
    app.add_systems(Update, cache_collider_locations);
    // app.add_systems(Update, setup_collision);
}

fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("textures/map/tilemap.ldtk").into(),
        ..Default::default()
    });
}

fn cache_collider_locations(
    mut level_walls: ResMut<LevelColliders>,
    mut level_messages: MessageReader<LevelEvent>,
    walls: Query<&GridCoords, With<Collider>>,
    ldtk_project_entities: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    let ldtk_project_entities = ldtk_project_entities.into_inner();

    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets.get(ldtk_project_entities).unwrap();
            let level = ldtk_project.get_raw_level_by_iid(level_iid.get()).unwrap();

            let collider_locations = walls.iter().copied().collect();

            let new_level_walls = LevelColliders {
                collider_locations,
                level_width: level.px_wid / GRID_SIZE,
                level_height: level.px_hei / GRID_SIZE,
            };

            *level_walls = new_level_walls;
        }
    }
}

// fn setup_collision(
//     mut commands: Commands,
//     new_entity_instances: Query<(Entity, &IntGridCell), Added<IntGridCell>>,
// ) {
//     for (entity, int_grid_cell) in new_entity_instances.iter() {
//         if int_grid_cell.value == 1 {}
//     }
// }
