use crate::game::{global::GameState, player::Player};
use bevy::prelude::*;

impl super::LightParameters for Player {}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        super::add_lights::<Player>.run_if(in_state(GameState::InGame)),
    );
}
