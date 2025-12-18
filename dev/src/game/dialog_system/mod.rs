use std::{collections::HashMap, fmt::Debug, rc::Rc};

use bevy::{asset::LoadedFolder, prelude::*};
use bladeink::story::Story;

use crate::game::custom_asset_types::ink_json::InkJson;

#[derive(Resource, Default)]
struct DialogsFolder(Handle<LoadedFolder>);

#[derive(Resource, Default)]
struct DialogsCache {
    dialogs: HashMap<String, String>,
}

#[derive(Message, Default)]
pub struct DialogTriggeredEvent {
    pub file_path: String,
    pub dialog_state: String,
    pub choice_index: Option<u8>,
}

#[derive(Debug)]
pub struct DialogChoice {
    pub index: u8,
    pub body: String,
}

#[derive(Message)]
pub struct DisplayCurrentDialogEvent {
    pub source_name: String,
    pub image_path: String,
    pub lines: Vec<String>,
    pub choices: Vec<DialogChoice>,
}

#[derive(Message)]
pub struct DialogEndedEvent;

pub fn plugin(app: &mut App) {
    app.add_message::<DialogTriggeredEvent>();
    app.add_message::<DisplayCurrentDialogEvent>();
    app.add_message::<DialogEndedEvent>();
    app.init_resource::<DialogsCache>();
    app.add_systems(Startup, load_dialog_folder);
    app.add_systems(Update, (cache_dialogs, next_dialog).chain());
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

                if let Some(dialog_json) = dialog_jsons.get(id) {
                    if let Some(path) = handle.path().unwrap().path().to_str() {
                        dialogs_cache
                            .dialogs
                            .insert(path.into(), dialog_json.string.clone().into());
                    }
                }
            }
        }
    }
}

fn next_dialog(
    mut dialog_event: MessageReader<DialogTriggeredEvent>,
    mut dialog_ui_event: MessageWriter<DisplayCurrentDialogEvent>,
    mut dialog_ended_event: MessageWriter<DialogEndedEvent>,
    dialogs_cache: Res<DialogsCache>,
) {
    for event in dialog_event.read() {
        if let Some(dialog) = dialogs_cache.dialogs.get(&event.file_path) {
            let mut story = match Story::new(dialog) {
                Ok(story) => story,
                Err(err) => panic!("Story can't be read: {:?}", err),
            };

            if !event.dialog_state.is_empty() {
                story
                    .load_state(&event.dialog_state)
                    .expect("Could not load story state");
            }

            if !story.can_continue() && story.get_current_choices().is_empty() {
                dialog_ended_event.write(DialogEndedEvent);
                return;
            }

            if let Some(choice_index) = event.choice_index {
                story
                    .choose_choice_index(choice_index as usize)
                    .expect("Could not set story choice");
            }

            let mut lines = Vec::new();

            while story.can_continue() {
                if let Ok(line) = story.cont() {
                    lines.push(line);
                }
            }

            let mut choices = vec![];

            for mut choice in story.get_current_choices().into_iter() {
                let choice = Rc::make_mut(&mut choice);

                choices.push(DialogChoice {
                    body: choice.text.clone(),
                    index: choice.index.clone().into_inner() as u8,
                });
            }

            let source_name = story
                .get_variable("sourceName")
                .unwrap()
                .get::<&str>()
                .unwrap()
                .to_string();

            let image_path = story
                .get_variable("imagePath")
                .unwrap()
                .get::<&str>()
                .unwrap()
                .to_string();

            dialog_ui_event.write(DisplayCurrentDialogEvent {
                source_name,
                image_path,
                lines,
                choices,
            });
        };
    }
}
