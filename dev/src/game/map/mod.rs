use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::global::GameState;

mod actionables;
pub mod inerts;
pub mod int_grid_objects;
pub mod npc;
pub mod utils;
mod zones;

pub const GRID_SIZE: i32 = 16;

#[derive(Message)]
pub struct ChangeLevel {
    identifier: String,
}

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

    app.add_message::<ChangeLevel>();

    app.add_systems(OnEnter(GameState::InGame), map_setup);
    app.add_systems(
        Update,
        (set_current_level_identifier, change_level).run_if(in_state(GameState::InGame)),
    );

    app.add_plugins(int_grid_objects::plugin);
    app.add_plugins(zones::plugin);
    app.add_plugins(npc::plugin);
    app.add_plugins(actionables::plugin);
    app.add_plugins(inerts::plugin);
}

pub fn change_level(
    mut events: MessageReader<ChangeLevel>,
    mut level_selection: ResMut<LevelSelection>,
) {
    for event in events.read() {
        *level_selection = LevelSelection::Identifier(event.identifier.clone());
    }
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
