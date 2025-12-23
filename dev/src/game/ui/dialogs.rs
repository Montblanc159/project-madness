use std::collections::HashMap;

use bevy::{
    color::palettes::css::BLACK,
    input_focus::{
        InputFocus, InputFocusVisible, directional_navigation::DirectionalNavigationMap,
    },
    math::CompassOctant,
    prelude::*,
};

use crate::game::{
    controls::{PlayerAction, PlayerInputs},
    dialog_system::{DialogChoice, DialogEndedEvent, DisplayCurrentDialogEvent, RunDialogEvent},
    ui::InputSelected,
};

#[derive(Component)]
struct DialogImageUi;

#[derive(Component)]
struct DialogSourceName;

#[derive(Component)]
struct DialogBodyUi;

#[derive(Component)]
struct DialogChoiceUi;

#[derive(Component)]
struct DialogChoiceIndex(usize);

#[derive(Component)]
struct DialogContainer;

#[derive(Component, Default)]
struct CurrentDialogLines(HashMap<u8, String>);

#[derive(Component, Default)]
struct CurrentDialogChoices(Vec<DialogChoice>);

#[derive(Component, Default)]
struct CurrentDialogImage(Handle<Image>);

#[derive(Component, Default)]
struct CurrentDialogSourceName(String);

#[derive(Component, Default)]
struct CurrentSourceEntity(Option<Entity>);

pub fn plugin(app: &mut App) {
    app.add_observer(select_choice);
    app.add_systems(Startup, (spawn_dialog_box, spawn_dialog_cache));
    app.add_systems(
        Update,
        (
            set_dialog_infos,
            update_image,
            update_source_name,
            set_dialog_line,
            update_dialog_choices,
            update_dialog_line,
            dialog_end_reached,
            hide_dialog,
            highlight_focused_element,
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
            DialogImageUi,
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

fn spawn_dialog_cache(mut commands: Commands) {
    commands.spawn((
        CurrentDialogLines(Default::default()),
        CurrentDialogImage(Default::default()),
        CurrentDialogSourceName(Default::default()),
        CurrentDialogChoices(Default::default()),
        CurrentSourceEntity(Default::default()),
    ));
}

fn set_dialog_infos(
    mut dialog_events: MessageReader<DisplayCurrentDialogEvent>,
    dialog_infos: Query<(
        &mut CurrentDialogLines,
        &mut CurrentDialogImage,
        &mut CurrentDialogSourceName,
        &mut CurrentDialogChoices,
        &mut CurrentSourceEntity,
    )>,
    dialog_container: Single<&mut Node, With<DialogContainer>>,
    asset_server: Res<AssetServer>,
) {
    let mut container = dialog_container.into_inner();

    for (
        mut dialog_lines,
        mut dialog_image,
        mut dialog_source_name,
        mut dialog_choices,
        mut current_entity,
    ) in dialog_infos
    {
        for event in dialog_events.read() {
            container.display = Display::Block;

            dialog_image.0 = asset_server.load(event.image_path.clone());
            dialog_source_name.0 = event.source_name.clone();

            current_entity.0 = Some(event.source_entity);

            for (index, line) in event.lines.iter().enumerate() {
                dialog_lines.0.insert(index as u8, line.clone());
            }

            for choice in &event.choices {
                dialog_choices.0.push(choice.clone());
            }
        }
    }
}

fn update_image(
    dialog_image: Single<&CurrentDialogImage, Changed<CurrentDialogImage>>,
    image_node: Single<&mut ImageNode, With<DialogImageUi>>,
) {
    let dialog_image = dialog_image.into_inner();
    let mut image_node = image_node.into_inner();

    image_node.image = dialog_image.0.clone();
}

fn update_source_name(
    dialog_source_names: Single<&CurrentDialogSourceName, Changed<CurrentDialogSourceName>>,
    node: Single<&mut Text, With<DialogSourceName>>,
) {
    let dialog_source_name = dialog_source_names.into_inner();
    let mut node = node.into_inner();

    node.0 = dialog_source_name.0.clone();
}

fn set_dialog_line(
    mut commands: Commands,
    nodes: Query<Entity, With<DialogBodyUi>>,
    dialog_lines: Single<&mut CurrentDialogLines, Changed<CurrentDialogLines>>,
    dialog_container: Single<Entity, With<DialogContainer>>,
) {
    let dialog_container = dialog_container.into_inner();
    let mut dialog_lines = dialog_lines.into_inner();

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
                    DialogBodyUi,
                ))
                .id();

            commands.entity(dialog_container).add_child(node);

            dialog_lines.0.remove(key);
        }
    }
}

fn update_dialog_line(
    mut commands: Commands,
    dialog_lines: Single<&mut CurrentDialogLines>,
    nodes: Query<Entity, With<DialogBodyUi>>,
    dialog_container: Single<Entity, With<DialogContainer>>,
    keys: Res<PlayerInputs>,
) {
    let dialog_container = dialog_container.into_inner();
    let mut dialog_lines = dialog_lines.into_inner();

    if keys.just_pressed_actions.contains(&PlayerAction::Activate) && !dialog_lines.0.is_empty() {
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
                    DialogBodyUi,
                ))
                .id();

            commands.entity(dialog_container).add_child(node);

            dialog_lines.0.remove(key);
        };
    }
}

fn update_dialog_choices(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
    dialog_nodes: Query<Entity, With<DialogBodyUi>>,
    dialog_infos: Single<(&CurrentDialogLines, &mut CurrentDialogChoices)>,
    node: Single<Entity, With<DialogContainer>>,
    keys: Res<PlayerInputs>,
) {
    let (dialog_lines, mut dialog_choices) = dialog_infos.into_inner();

    if dialog_lines.0.is_empty()
        && !dialog_choices.0.is_empty()
        && keys.just_pressed_actions.contains(&PlayerAction::Activate)
    {
        for dialog_node in dialog_nodes {
            commands.entity(dialog_node).despawn();
        }

        let node = node.into_inner();

        let mut choices: Vec<Entity> = vec![];

        for choice in dialog_choices.0.clone() {
            let choice_node = commands
                .spawn((
                    Text::new(choice.body),
                    Node {
                        width: percent(100),
                        ..Default::default()
                    },
                    TextFont {
                        font_size: super::DEFAULT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(BLACK.into()),
                    DialogChoiceIndex(choice.index),
                    DialogChoiceUi,
                    Name::new(format!("choice::index({})", choice.index)),
                ))
                .id();

            let choice_index = dialog_choices
                .0
                .iter()
                .position(|c| c.index == choice.index)
                .unwrap();

            dialog_choices.0.remove(choice_index);

            commands.entity(node).add_child(choice_node);

            choices.push(choice_node);
        }

        directional_nav_map.add_looping_edges(&choices, CompassOctant::South);

        input_focus.set(choices[0]);
    }
}

fn highlight_focused_element(
    input_focus: Res<InputFocus>,
    // While this isn't strictly needed for the example,
    // we're demonstrating how to be a good citizen by respecting the `InputFocusVisible` resource.
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(Entity, &mut TextFont), With<DialogChoiceUi>>,
) {
    for (entity, mut text) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            text.font_size = super::DEFAULT_FONT_SIZE + 5.
        } else {
            text.font_size = super::DEFAULT_FONT_SIZE
        }
    }
}

fn select_choice(
    event: On<InputSelected>,
    mut commands: Commands,
    mut dialog_event: MessageWriter<RunDialogEvent>,
    choices: Query<(Entity, &DialogChoiceIndex), With<DialogChoiceUi>>,
    source_entity: Single<&CurrentSourceEntity>,
) {
    if let Ok((_, choice_index)) = choices.get(event.entity)
        && let Some(source_entity) = source_entity.into_inner().0
    {
        dialog_event.write(RunDialogEvent {
            source_entity: source_entity,
            choice_index: Some(choice_index.0),
        });
    }

    for (entity, _) in choices {
        commands.entity(entity).despawn();
    }
}

fn dialog_end_reached(
    choices: Single<&CurrentDialogChoices>,
    lines: Single<&CurrentDialogLines>,
    mut dialog_ended_event: MessageWriter<DialogEndedEvent>,
    keys: Res<PlayerInputs>,
) {
    if choices.into_inner().0.is_empty()
        && lines.into_inner().0.is_empty()
        && keys.just_pressed_actions.contains(&PlayerAction::Activate)
    {
        dialog_ended_event.write(DialogEndedEvent);
    }
}

fn hide_dialog(
    mut dialog_events: MessageReader<DialogEndedEvent>,
    dialog_container: Single<&mut Node, With<DialogContainer>>,
) {
    let mut container = dialog_container.into_inner();

    for _ in dialog_events.read() {
        container.display = Display::None;
    }
}
