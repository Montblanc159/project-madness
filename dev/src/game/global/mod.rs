use bevy::prelude::*;

use crate::game::map::ChangeLevel;

// pub fn plugin(app: &mut App) {}

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
