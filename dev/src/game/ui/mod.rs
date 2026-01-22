use bevy::{
    input_focus::{
        InputDispatchPlugin, InputFocus, InputFocusVisible,
        directional_navigation::{
            DirectionalNavigation, DirectionalNavigationMap, DirectionalNavigationPlugin,
        },
    },
    math::CompassOctant,
    prelude::*,
};

use crate::game::controls::{PlayerAction, PlayerInputs};

pub mod dialogs;
mod menu;

pub const DEFAULT_FONT_SIZE: f32 = 45.;
pub const DEFAULT_PADDING: u8 = 25;

#[derive(EntityEvent)]
struct InputSelected {
    entity: Entity,
}

pub fn plugin(app: &mut App) {
    app.add_plugins((
        InputDispatchPlugin,
        DirectionalNavigationPlugin,
        dialogs::plugin,
        menu::plugin,
    ));
    app.insert_resource(InputFocusVisible(true));
    app.add_systems(Update, (navigate, interact_with_focused_input));
}

fn navigate(
    action_state: Res<PlayerInputs>,
    mut directional_navigation: DirectionalNavigation,
    directional_nav_map: Res<DirectionalNavigationMap>,
) {
    if directional_nav_map.neighbors.is_empty() {
        return;
    }

    // If the user is pressing both left and right, or up and down,
    // we should not move in either direction.
    let net_east_west = action_state
        .just_pressed_actions
        .contains(&PlayerAction::Right) as i8
        - action_state
            .just_pressed_actions
            .contains(&PlayerAction::Left) as i8;

    let net_north_south = action_state
        .just_pressed_actions
        .contains(&PlayerAction::Up) as i8
        - action_state
            .just_pressed_actions
            .contains(&PlayerAction::Down) as i8;

    // Compute the direction that the user is trying to navigate in
    let maybe_direction = match (net_east_west, net_north_south) {
        (0, 0) => None,
        (0, 1) => Some(CompassOctant::North),
        (1, 1) => Some(CompassOctant::NorthEast),
        (1, 0) => Some(CompassOctant::East),
        (1, -1) => Some(CompassOctant::SouthEast),
        (0, -1) => Some(CompassOctant::South),
        (-1, -1) => Some(CompassOctant::SouthWest),
        (-1, 0) => Some(CompassOctant::West),
        (-1, 1) => Some(CompassOctant::NorthWest),
        _ => None,
    };

    if let Some(direction) = maybe_direction {
        match directional_navigation.navigate(direction) {
            // In a real game, you would likely want to play a sound or show a visual effect
            // on both successful and unsuccessful navigation attempts
            Ok(entity) => {
                println!("Navigated {direction:?} successfully. {entity} is now focused.");
            }
            Err(e) => println!("Navigation failed: {e}"),
        }
    }
}

fn interact_with_focused_input(
    mut commands: Commands,
    mut directional_nav_map: ResMut<DirectionalNavigationMap>,
    action_state: Res<PlayerInputs>,
    input_focus: Res<InputFocus>,
) {
    if action_state
        .just_pressed_actions
        .contains(&PlayerAction::Activate)
        && let Some(entity) = input_focus.0
    {
        directional_nav_map.clear();
        commands.trigger(InputSelected { entity });
    }
}
