use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod actionables;
pub mod colliders;
pub mod inerts;
pub mod npc;
pub mod utils;
mod zones;

pub const GRID_SIZE: i32 = 16;

#[derive(Resource, Default, Debug)]
pub struct CurrentLevelInfos {
    pub identifier: String,
    coming_from: Option<String>,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(CurrentLevelInfos {
        identifier: "Level_0".into(),
        ..Default::default()
    });
    app.insert_resource(LevelSelection::index(0));

    app.add_systems(Startup, map_setup);
    app.add_systems(Update, set_current_level_identifier);
    app.add_plugins(colliders::plugin);
    app.add_plugins(zones::plugin);
    app.add_plugins(npc::plugin);
    app.add_plugins(actionables::plugin);
    // app.add_plugins(inerts::plugin);
}

fn map_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("textures/map/tilemap.ldtk").into(),
        ..Default::default()
    });
}

fn set_current_level_identifier(
    ldtk_project_entity: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    level_selection: Res<LevelSelection>,
    mut level_infos: ResMut<CurrentLevelInfos>,
    mut level_messages: MessageReader<LevelEvent>,
) {
    let ldtk_project_entity = ldtk_project_entity.into_inner();

    let ldtk_project = ldtk_project_assets.get(ldtk_project_entity).unwrap();
    let level = ldtk_project
        .find_raw_level_by_level_selection(&level_selection)
        .unwrap();

    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(_) = level_event {
            *level_infos = CurrentLevelInfos {
                coming_from: Some(level_infos.identifier.clone()),
                identifier: level.identifier.clone(),
            };
        }
    }
}
