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
struct DialogLinesUi;

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
struct CurrentDialogChoiceIndex(Option<usize>);

#[derive(Component, Default)]
struct CurrentDialogImage(Handle<Image>);

#[derive(Component, Default)]
struct CurrentDialogSourceName(String);

#[derive(Component, Default)]
struct CurrentSourceEntity(Option<Entity>);

pub fn plugin(app: &mut App) {
    app.add_observer(set_choice_index);
    app.add_systems(Startup, (spawn_dialog_box, spawn_dialog_cache));
    app.add_systems(
        Update,
        (
            clean_dialog_container,
            set_dialog_cache,
            update_image,
            update_source_name,
            set_dialog_line,
            update_dialog_choices,
            highlight_focused_element,
            update_dialog_line,
            fetch_next_dialog_block.run_if(dialog_end_reached),
            end_dialog,
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
        CurrentDialogChoiceIndex(Default::default()),
    ));
}

fn set_dialog_cache(
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

fn clean_dialog_container(
    mut commands: Commands,
    lines: Query<Entity, With<DialogLinesUi>>,
    choices: Query<Entity, With<DialogChoiceUi>>,
    keys: Res<PlayerInputs>,
) {
    if keys.just_pressed_actions.contains(&PlayerAction::Activate) {
        for line in lines {
            commands.entity(line).despawn();
        }

        for choice in choices {
            commands.entity(choice).despawn();
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
    dialog_lines: Single<&mut CurrentDialogLines, Changed<CurrentDialogLines>>,
    dialog_container: Single<Entity, With<DialogContainer>>,
) {
    let dialog_container = dialog_container.into_inner();
    let mut dialog_lines = dialog_lines.into_inner();

    if let Some(key) = dialog_lines.0.clone().keys().min() {
        if *key == 0 {
            let node = commands
                .spawn((
                    Text::new(dialog_lines.0[key].clone()),
                    TextFont {
                        font_size: super::DEFAULT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(BLACK.into()),
                    DialogLinesUi,
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
    dialog_container: Single<Entity, With<DialogContainer>>,
    keys: Res<PlayerInputs>,
) {
    let dialog_container = dialog_container.into_inner();
    let mut dialog_lines = dialog_lines.into_inner();

    if keys.just_pressed_actions.contains(&PlayerAction::Activate) && !dialog_lines.0.is_empty() {
        if let Some(key) = dialog_lines.0.clone().keys().min() {
            let node = commands
                .spawn((
                    Text::new(dialog_lines.0[key].clone()),
                    TextFont {
                        font_size: super::DEFAULT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(BLACK.into()),
                    DialogLinesUi,
                ))
                .id();

            commands.entity(dialog_container).add_child(node);

            dialog_lines.0.remove(key);
        };
    }
}

fn dialog_end_reached(choices: Query<&DialogChoiceUi>, lines: Query<&DialogLinesUi>) -> bool {
    choices.is_empty() && lines.is_empty()
}

fn fetch_next_dialog_block(
    mut dialog_event: MessageWriter<RunDialogEvent>,
    source_entity: Single<&CurrentSourceEntity>,
    choice_index: Single<&mut CurrentDialogChoiceIndex>,
    keys: Res<PlayerInputs>,
) {
    if let Some(source_entity) = source_entity.into_inner().0
        && keys.just_pressed_actions.contains(&PlayerAction::Activate)
    {
        let mut choice_index = choice_index.into_inner();

        println!("Fetching next dialog block");
        dialog_event.write(RunDialogEvent {
            source_entity,
            choice_index: choice_index.0,
        });

        choice_index.0 = None;
    }
}

fn update_dialog_choices(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
    dialog_infos: Single<(&CurrentDialogLines, &mut CurrentDialogChoices)>,
    node: Single<Entity, With<DialogContainer>>,
    keys: Res<PlayerInputs>,
) {
    let (dialog_lines, mut dialog_choices) = dialog_infos.into_inner();

    if dialog_lines.0.is_empty()
        && !dialog_choices.0.is_empty()
        && keys.just_pressed_actions.contains(&PlayerAction::Activate)
    {
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

            commands.entity(node).add_child(choice_node);

            choices.push(choice_node);
        }

        dialog_choices.0.clear();

        directional_nav_map.add_looping_edges(&choices, CompassOctant::South);

        input_focus.set(choices[0]);
    }
}

fn highlight_focused_element(
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
    mut query: Query<(Entity, &mut TextFont), With<DialogChoiceUi>>,
) {
    for (entity, mut text) in query.iter_mut() {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            text.font_size = super::DEFAULT_FONT_SIZE + 5.;
        } else {
            text.font_size = super::DEFAULT_FONT_SIZE;
        }
    }
}

fn set_choice_index(
    event: On<InputSelected>,
    mut commands: Commands,
    choices: Query<(Entity, &DialogChoiceIndex), With<DialogChoiceUi>>,
    current_choice_index: Single<&mut CurrentDialogChoiceIndex>,
) {
    if let Ok((_, choice_index)) = choices.get(event.entity) {
        current_choice_index.into_inner().0 = Some(choice_index.0);
    }

    for (entity, _) in choices {
        commands.entity(entity).despawn();
    }
}

fn end_dialog(
    mut dialog_events: MessageReader<DialogEndedEvent>,
    dialog_container: Single<&mut Node, With<DialogContainer>>,
) {
    let mut container = dialog_container.into_inner();

    for _ in dialog_events.read() {
        container.display = Display::None;
    }
}
