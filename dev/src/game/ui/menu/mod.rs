use bevy::{
    input_focus::{
        InputFocus, InputFocusVisible, directional_navigation::DirectionalNavigationMap,
    },
    math::CompassOctant,
    prelude::*,
};

use crate::game::{
    global::{GameState, StartGame},
    ui::{DEFAULT_FONT_SIZE, DEFAULT_PADDING, InputSelected},
};

#[derive(Component)]
struct MenuContainer;

#[derive(Component)]
struct MenuOption;

pub fn plugin(app: &mut App) {
    app.add_observer(react_to_player_selection);
    app.add_systems(OnEnter(GameState::Menu), (spawn_menu, add_nav_map).chain());
    app.add_systems(
        Update,
        highlight_focused_element.run_if(in_state(GameState::Menu)),
    );
}

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                bottom: px(0),
                left: px(0),
                right: px(0),
                top: px(0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,

                ..Default::default()
            },
            MenuContainer,
            DespawnOnExit(GameState::Menu),
        ))
        .with_children(|parent| {
            for option in ["Play", "Settings"] {
                parent
                    .spawn((
                        Name::new(option),
                        Node {
                            width: px(250),
                            height: px(85),
                            border: UiRect::all(px(5)),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            padding: UiRect {
                                left: px(DEFAULT_PADDING),
                                right: px(DEFAULT_PADDING),
                                top: px(DEFAULT_PADDING),
                                bottom: px(DEFAULT_PADDING),
                            },
                            ..default()
                        },
                        BorderColor::all(Color::WHITE),
                        MenuOption,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(option),
                            TextFont {
                                font_size: DEFAULT_FONT_SIZE,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            // TextShadow::default(),
                        ));
                    });
            }
        });
}

fn add_nav_map(
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    mut input_focus: ResMut<InputFocus>,
    options_entity: Query<Entity, Added<MenuOption>>,
) {
    let mut options = [Entity::PLACEHOLDER; 2];

    for (index, entity) in options_entity.iter().enumerate() {
        options[index] = entity;
    }

    directional_nav_map.clear();
    directional_nav_map.add_looping_edges(&options, CompassOctant::South);

    input_focus.set(options[0]);
}

fn highlight_focused_element(
    mut commands: Commands,
    options: Query<(Entity, &Children), With<MenuOption>>,
    mut text_colors: Query<&mut TextColor>,
    input_focus: Res<InputFocus>,
    input_focus_visible: Res<InputFocusVisible>,
) {
    for (entity, children) in options {
        if input_focus.0 == Some(entity) && input_focus_visible.0 {
            commands
                .entity(entity)
                .insert(BackgroundColor(Color::WHITE));
            for child in children {
                if let Ok(mut text_color) = text_colors.get_mut(*child) {
                    text_color.0 = Color::BLACK;
                }
            }
        } else {
            commands.entity(entity).remove::<BackgroundColor>();
            for child in children {
                if let Ok(mut text_color) = text_colors.get_mut(*child) {
                    text_color.0 = Color::WHITE;
                }
            }
        }
    }
}

fn react_to_player_selection(
    event: On<InputSelected>,
    mut play_event: MessageWriter<StartGame>,
    options: Query<&Name, With<MenuOption>>,
) {
    if let Ok(option) = options.get(event.entity) {
        if option.as_str() == "Play" {
            play_event.write(StartGame);
        }
    }
}
