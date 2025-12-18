use std::collections::HashMap;

use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::game::dialog_system::{DialogEndedEvent, DisplayCurrentDialogEvent};

#[derive(Component)]
struct DialogImage;

#[derive(Component)]
struct DialogSourceName;

#[derive(Component)]
struct DialogBody;

#[derive(Component)]
struct DialogChoice;

#[derive(Component)]
struct DialogChoiceIndex(u8);

#[derive(Component)]
struct DialogContainer;

#[derive(Resource, Default)]
struct CurrentDialogLines(HashMap<u8, String>);

#[derive(Resource, Default)]
struct CurrentDialogChoices(HashMap<u8, String>);

#[derive(Resource, Default)]
struct CurrentDialogImage(Handle<Image>);

#[derive(Resource, Default)]
struct CurrentDialogSourceName(String);

pub fn plugin(app: &mut App) {
    app.init_resource::<CurrentDialogLines>();
    app.init_resource::<CurrentDialogChoices>();
    app.init_resource::<CurrentDialogImage>();
    app.init_resource::<CurrentDialogSourceName>();
    app.add_systems(Startup, spawn_dialog_box);
    app.add_systems(
        Update,
        (
            set_dialog_infos,
            update_image,
            update_source_name,
            set_dialog_line,
            update_dialog_choices,
            update_dialog_line,
            hide_dialog,
        )
            .chain(),
    );
}

fn spawn_dialog_box(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut entity = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(0),
            left: px(0),
            right: px(0),
            aspect_ratio: Some(240. / 64.),
            display: Display::None,
            padding: UiRect {
                left: px(super::DEFAULT_PADDING),
                right: px(super::DEFAULT_PADDING),
                top: px(super::DEFAULT_PADDING - 7),
                bottom: px(super::DEFAULT_PADDING),
            },
            ..Default::default()
        },
        ImageNode {
            image: asset_server.load("textures/ui/dialog-box.png"),
            ..Default::default()
        },
        DialogContainer,
    ));

    entity.with_children(|child_commands| {
        child_commands.spawn((
            ImageNode {
                ..Default::default()
            },
            Node {
                // aspect_ratio: Some(64. / 64.),
                ..Default::default()
            },
            TextColor(BLACK.into()),
            DialogImage,
        ));

        child_commands.spawn((
            Text::new(""),
            TextFont {
                font_size: super::DEFAULT_FONT_SIZE,
                ..default()
            },
            TextColor(BLACK.into()),
            DialogSourceName,
        ));
    });
}

fn set_dialog_infos(
    mut dialog_events: MessageReader<DisplayCurrentDialogEvent>,
    mut dialog_lines: ResMut<CurrentDialogLines>,
    mut dialog_image: ResMut<CurrentDialogImage>,
    mut dialog_source_name: ResMut<CurrentDialogSourceName>,
    mut dialog_choices: ResMut<CurrentDialogChoices>,
    dialog_container: Single<&mut Node, With<DialogContainer>>,
    asset_server: Res<AssetServer>,
) {
    let mut container = dialog_container.into_inner();

    for event in dialog_events.read() {
        container.display = Display::Block;

        dialog_image.0 = asset_server.load(event.image_path.clone());
        dialog_source_name.0 = event.source_name.clone();

        for (index, line) in event.lines.iter().enumerate() {
            dialog_lines.0.insert(index as u8, line.clone());
        }

        for choice in &event.choices {
            dialog_choices.0.insert(choice.index, choice.body.clone());
        }
    }
}

fn update_image(
    dialog_image: Res<CurrentDialogImage>,
    image_node: Single<&mut ImageNode, With<DialogImage>>,
) {
    if dialog_image.is_changed() {
        let mut image_node = image_node.into_inner();

        image_node.image = dialog_image.0.clone();
    }
}

fn update_source_name(
    dialog_source_name: Res<CurrentDialogSourceName>,
    node: Single<&mut Text, With<DialogSourceName>>,
) {
    if dialog_source_name.is_changed() {
        let mut node = node.into_inner();

        node.0 = dialog_source_name.0.clone();
    }
}

fn set_dialog_line(
    mut commands: Commands,
    mut dialog_lines: ResMut<CurrentDialogLines>,
    nodes: Query<Entity, With<DialogBody>>,
    dialog_container: Single<Entity, With<DialogContainer>>,
) {
    if let Some(key) = dialog_lines.0.clone().keys().min() {
        if *key == 0 {
            for node in nodes {
                commands.entity(node).despawn();
            }

            let node = commands
                .spawn((
                    Text::new(dialog_lines.0[key].clone()),
                    TextFont {
                        font_size: super::DEFAULT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(BLACK.into()),
                    DialogBody,
                ))
                .id();

            commands
                .entity(dialog_container.into_inner())
                .add_child(node);

            dialog_lines.0.remove(key);
        }
    }
}

fn update_dialog_line(
    mut commands: Commands,
    mut dialog_lines: ResMut<CurrentDialogLines>,
    nodes: Query<Entity, With<DialogBody>>,
    dialog_container: Single<Entity, With<DialogContainer>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) && !dialog_lines.0.is_empty() {
        for node in nodes {
            commands.entity(node).despawn();
        }

        if let Some(key) = dialog_lines.0.clone().keys().min() {
            let node = commands
                .spawn((
                    Text::new(dialog_lines.0[key].clone()),
                    TextFont {
                        font_size: super::DEFAULT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(BLACK.into()),
                    DialogBody,
                ))
                .id();

            commands
                .entity(dialog_container.into_inner())
                .add_child(node);

            dialog_lines.0.remove(key);
        };
    }
}

fn update_dialog_choices(
    mut commands: Commands,
    mut dialog_choices: ResMut<CurrentDialogChoices>,
    dialog_nodes: Query<Entity, With<DialogBody>>,
    dialog_lines: Res<CurrentDialogLines>,
    node: Single<Entity, With<DialogContainer>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if dialog_lines.0.is_empty()
        && !dialog_choices.0.is_empty()
        && keys.just_pressed(KeyCode::Space)
    {
        for dialog_node in dialog_nodes {
            commands.entity(dialog_node).despawn();
        }

        let node = node.into_inner();

        for (index, body) in dialog_choices.0.clone() {
            let choice_node = commands
                .spawn((
                    Text::new(body),
                    Node {
                        width: percent(100),
                        ..Default::default()
                    },
                    TextFont {
                        font_size: super::DEFAULT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(BLACK.into()),
                    DialogChoiceIndex(index),
                    DialogChoice,
                ))
                .id();

            dialog_choices.0.remove(&index);

            commands.entity(node).add_child(choice_node);
        }
    }
}

fn hide_dialog(
    mut dialog_events: MessageReader<DialogEndedEvent>,
    dialog_container: Single<&mut Node, With<DialogContainer>>,
) {
    if !dialog_events.read().len() == 0 {
        let mut container = dialog_container.into_inner();
        container.display = Display::None;
    }
}
