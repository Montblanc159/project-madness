use std::{collections::HashMap, fmt::Debug};

use bevy::{asset::LoadedFolder, prelude::*};

use crate::game::{
    custom_asset_types::ink_json::InkJson,
    map::npc::{AvatarFilePath, NpcName},
};

mod helpers;

#[derive(Resource, Default)]
struct DialogsFolder(Handle<LoadedFolder>);

#[derive(Resource, Default)]
struct DialogsCache {
    dialogs: HashMap<String, String>,
}

#[derive(Component, Default)]
pub struct DialogFilePath(pub String);

#[derive(Component, Default)]
pub struct DialogState(pub String);

#[derive(Component, Default)]
pub struct DialogKnot(pub String);

#[derive(Message)]
pub struct RunDialogEvent {
    pub source_entity: Entity,
    pub choice_index: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct DialogChoice {
    pub index: usize,
    pub body: String,
}

#[derive(Message)]
pub struct DisplayCurrentDialogEvent {
    pub source_entity: Entity,
    pub source_name: String,
    pub image_path: String,
    pub lines: Vec<String>,
    pub choices: Vec<DialogChoice>,
}

#[derive(Message)]
pub struct UpdateDialogStateEvent {
    source_entity: Entity,
    dialog_state: String,
}

#[derive(Message)]
pub struct DialogEndedEvent;

pub fn plugin(app: &mut App) {
    app.add_message::<RunDialogEvent>();
    app.add_message::<DisplayCurrentDialogEvent>();
    app.add_message::<DialogEndedEvent>();
    app.add_message::<UpdateDialogStateEvent>();
    app.init_resource::<DialogsCache>();
    app.add_systems(Startup, load_dialog_folder);
    app.add_systems(
        Update,
        (cache_dialogs, run_dialog, update_dialog_state).chain(),
    );
}

fn load_dialog_folder(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.insert_resource(DialogsFolder(asset_server.load_folder("dialogs")));
}

fn cache_dialogs(
    mut events: MessageReader<AssetEvent<LoadedFolder>>,
    dialogs_folder: Res<DialogsFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    dialog_jsons: ResMut<Assets<InkJson>>,
    mut dialogs_cache: ResMut<DialogsCache>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&dialogs_folder.0) {
            let loaded_folder = loaded_folders.get(&dialogs_folder.0).unwrap();

            for handle in loaded_folder.handles.iter() {
                let id = handle.id().typed::<InkJson>();

                if let Some(dialog_json) = dialog_jsons.get(id)
                    && let Some(path) = handle.path().unwrap().path().to_str()
                {
                    dialogs_cache
                        .dialogs
                        .insert(path.into(), dialog_json.string.clone());
                }
            }
        }
    }
}

fn run_dialog(
    mut dialog_event: MessageReader<RunDialogEvent>,
    mut dialog_ui_event: MessageWriter<DisplayCurrentDialogEvent>,
    mut update_entity_event: MessageWriter<UpdateDialogStateEvent>,
    mut dialog_ended_event: MessageWriter<DialogEndedEvent>,
    entities: Query<(
        &DialogFilePath,
        &DialogState,
        &DialogKnot,
        &NpcName,
        &AvatarFilePath,
    )>,
    dialogs_cache: Res<DialogsCache>,
) {
    for event in dialog_event.read() {
        if let Ok((file_path, dialog_state, dialog_knot, name, avatar_file_path)) =
            entities.get(event.source_entity)
            && let Some(dialog_file) = dialogs_cache.dialogs.get(&file_path.0)
        {
            let mut story =
                helpers::get_story_with_state(dialog_file, &dialog_state.0, &dialog_knot.0);

            if let Some(choice_index) = event.choice_index {
                story
                    .choose_choice_index(choice_index)
                    .expect("Could not set story choice");
            }

            let lines = helpers::get_lines(&mut story);

            if let Ok(dialog_state) = story.save_state() {
                update_entity_event.write(UpdateDialogStateEvent {
                    source_entity: event.source_entity,
                    dialog_state,
                });
            }

            let choices = helpers::get_choices(&story);

            if lines.is_empty() && choices.is_empty() {
                dialog_ended_event.write(DialogEndedEvent);
            } else {
                dialog_ui_event.write(DisplayCurrentDialogEvent {
                    source_entity: event.source_entity,
                    source_name: name.0.clone(),
                    image_path: avatar_file_path.0.clone(),
                    lines,
                    choices,
                });
            }
        };
    }
}

fn update_dialog_state(
    mut entities: Query<&mut DialogState>,
    mut update_event: MessageReader<UpdateDialogStateEvent>,
) {
    for event in update_event.read() {
        if let Ok(mut dialog_state) = entities.get_mut(event.source_entity) {
            dialog_state.0 = event.dialog_state.clone();
        }
    }
}
