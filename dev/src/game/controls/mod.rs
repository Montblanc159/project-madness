use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl PlayerAction {
    fn variants() -> Vec<Self> {
        vec![
            PlayerAction::Up,
            PlayerAction::Down,
            PlayerAction::Left,
            PlayerAction::Right,
            PlayerAction::Activate,
        ]
    }

    fn keycode(&self) -> KeyCode {
        match self {
            PlayerAction::Up => KeyCode::KeyW,
            PlayerAction::Down => KeyCode::KeyS,
            PlayerAction::Left => KeyCode::KeyA,
            PlayerAction::Right => KeyCode::KeyD,
            PlayerAction::Activate => KeyCode::Space,
        }
    }

    fn gamepad_button(&self) -> GamepadButton {
        match self {
            PlayerAction::Up => GamepadButton::DPadUp,
            PlayerAction::Down => GamepadButton::DPadDown,
            PlayerAction::Left => GamepadButton::DPadLeft,
            PlayerAction::Right => GamepadButton::DPadRight,
            PlayerAction::Activate => GamepadButton::South,
        }
    }
}

#[derive(Default, Resource)]
pub struct PlayerInputs {
    pub pressed_actions: HashSet<PlayerAction>,
    pub just_pressed_actions: HashSet<PlayerAction>,
    pub just_released_actions: HashSet<PlayerAction>,
}

pub fn plugin(app: &mut App) {
    app.init_resource::<PlayerInputs>();
    app.add_systems(
        PreUpdate,
        (
            process_pressed_inputs,
            process_just_released_inputs,
            process_just_pressed_inputs,
        ),
    );
}

fn process_pressed_inputs(
    mut player_input: ResMut<PlayerInputs>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepad_input: Query<&Gamepad>,
) {
    player_input.pressed_actions.clear();

    for action in PlayerAction::variants() {
        if keyboard_input.pressed(action.keycode()) {
            player_input.pressed_actions.insert(action);
        }
    }

    for gamepad in gamepad_input.iter() {
        for action in PlayerAction::variants() {
            if gamepad.pressed(action.gamepad_button()) {
                player_input.pressed_actions.insert(action);
            }
        }
    }
}

fn process_just_pressed_inputs(
    mut player_input: ResMut<PlayerInputs>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepad_input: Query<&Gamepad>,
) {
    player_input.just_pressed_actions.clear();

    for action in PlayerAction::variants() {
        if keyboard_input.just_pressed(action.keycode()) {
            player_input.just_pressed_actions.insert(action);
        }
    }

    for gamepad in gamepad_input.iter() {
        for action in PlayerAction::variants() {
            if gamepad.just_pressed(action.gamepad_button()) {
                player_input.just_pressed_actions.insert(action);
            }
        }
    }
}

fn process_just_released_inputs(
    mut player_input: ResMut<PlayerInputs>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepad_input: Query<&Gamepad>,
) {
    player_input.just_released_actions.clear();

    for action in PlayerAction::variants() {
        if keyboard_input.just_released(action.keycode()) {
            player_input.just_released_actions.insert(action);
        }
    }

    for gamepad in gamepad_input.iter() {
        for action in PlayerAction::variants() {
            if gamepad.just_released(action.gamepad_button()) {
                player_input.just_released_actions.insert(action);
            }
        }
    }
}
