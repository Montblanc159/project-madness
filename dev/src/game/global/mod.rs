use bevy::prelude::*;

use crate::game::map::ChangeLevel;

/// Global game state
#[derive(States, Default, Debug, Clone, Hash, Eq, PartialEq)]
pub enum GameState {
    Menu,
    #[default]
    InGame,
}

#[derive(Message)]
pub struct StartGame;

// #[derive(States, Default, Debug, Clone, Hash, Eq, PartialEq)]
// pub enum InGameState {
//     #[default]
//     Playing,
//     Menu,
//     Paused,
//     Loading,
// }

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.add_message::<StartGame>();
    app.add_systems(Update, start_game.run_if(not(in_state(GameState::InGame))));
}

/// Despawn an entity matching a specific component
/// when player changes level
pub fn despawn_entity_on_level_change<T: Component>(
    mut commands: Commands,
    mut events: MessageReader<ChangeLevel>,
    entities: Query<Entity, With<T>>,
) {
    for _ in events.read() {
        for entity in entities {
            commands.entity(entity).despawn();
        }
    }
}

// pub fn despawn_entities<T: Component>(mut commands: Commands, entities: Query<Entity, With<T>>) {
//     for entity in entities {
//         commands.entity(entity).despawn();
//     }
// }

// pub fn load_game(mut next_state: ResMut<NextState<GameState>>) {
//     next_state.set(GameState::Loading);
// }

// pub fn back_to_menu(mut next_state: ResMut<NextState<GameState>>) {
//     next_state.set(GameState::Menu);
// }

pub fn start_game(
    mut events: MessageReader<StartGame>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        next_state.set(GameState::InGame);
    }
}
