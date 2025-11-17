use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(LevelSelection::index(0));
    app.add_systems(Startup, map_setup);
    // app.add_systems(Update, setup_collision);
}

fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("textures/map/tilemap.ldtk").into(),
        ..Default::default()
    });
}

// fn setup_collision(
//     mut commands: Commands,
//     new_entity_instances: Query<(Entity, &IntGridCell), Added<IntGridCell>>,
// ) {
//     for (entity, int_grid_cell) in new_entity_instances.iter() {
//         if int_grid_cell.value == 1 {}
//     }
// }
