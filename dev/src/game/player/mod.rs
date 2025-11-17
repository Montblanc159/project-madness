use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::camera::CameraTarget;
use super::tick::MainTick;

const GRID_SIZE: u8 = 16;

#[derive(Default, Component)]
struct Player;

#[derive(Default, Component)]
struct Velocity {
    value: GridCoords,
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            process_player,
            player_movement_input,
            translate_transform_to_grid_coords,
            translate_grid_coords_entities,
            move_player,
        )
            .chain(),
    );
}

fn process_player(
    mut commands: Commands,
    new_entity_instances: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    server: Res<AssetServer>,
) {
    for (entity, entity_instance) in new_entity_instances.iter() {
        if entity_instance.identifier == "Player".to_string() {
            commands.entity(entity).insert((
                Player,
                CameraTarget,
                AseSlice {
                    name: "player_idle".into(),
                    aseprite: server.load("textures/player/player.aseprite"),
                },
                Sprite::default(),
                GridCoords {
                    ..Default::default()
                },
                Velocity {
                    ..Default::default()
                },
            ));
        }
    }
}

fn player_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    player_velocity: Single<&mut Velocity, With<Player>>,
) {
    let mut velocity = player_velocity.into_inner();

    velocity.value = GridCoords {
        ..Default::default()
    };

    if keys.pressed(KeyCode::KeyA) {
        velocity.value = GridCoords {
            x: -1,
            ..Default::default()
        }
    } else if keys.pressed(KeyCode::KeyD) {
        velocity.value = GridCoords {
            x: 1,
            ..Default::default()
        }
    } else if keys.pressed(KeyCode::KeyW) {
        velocity.value = GridCoords {
            y: 1,
            ..Default::default()
        }
    } else if keys.pressed(KeyCode::KeyS) {
        velocity.value = GridCoords {
            y: -1,
            ..Default::default()
        }
    }
}

fn move_player(
    main_tick: Res<MainTick>,
    mut query: Query<(&mut GridCoords, &Velocity), With<Player>>,
) {
    // let delta_secs = time.delta_secs();

    for (mut player_grid_coords, velocity) in query.iter_mut() {
        if main_tick.timer.just_finished() {
            let destination = *player_grid_coords + velocity.value;
            *player_grid_coords = destination;
        }
    }
}

fn translate_transform_to_grid_coords(
    mut grid_coords_entities: Query<(&Transform, &mut GridCoords), Added<GridCoords>>,
) {
    for (transform, mut grid_coords) in grid_coords_entities.iter_mut() {
        *grid_coords = bevy_ecs_ldtk::utils::translation_to_grid_coords(
            Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            },
            IVec2::splat(GRID_SIZE.into()),
        );
    }
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
            *grid_coords,
            IVec2::splat(GRID_SIZE.into()),
        )
        .extend(transform.translation.z);
    }
}
