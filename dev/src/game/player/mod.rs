use std::time::Duration;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tweening::*;

use super::camera::CameraTarget;
use super::tick::TICK_DELTA;

const GRID_SIZE: u8 = 16;
const JITTER_THRESHOLD: f32 = 0.015;

#[derive(Resource)]
pub struct WalkCycleTimer {
    pub timer: Timer,
}

#[derive(Default, Component)]
struct Player;

#[derive(Default, Component)]
struct Velocity {
    value: GridCoords,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init_walk_cycle_timer);
    app.add_systems(
        Update,
        (
            process_player,
            translate_transform_to_grid_coords,
            player_movement_input,
            move_player,
            translate_grid_coords_entities,
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

fn init_walk_cycle_timer(mut commands: Commands) {
    commands.insert_resource(WalkCycleTimer {
        timer: Timer::new(Duration::from_secs_f32(TICK_DELTA), TimerMode::Once),
    })
}

fn player_movement_input(
    keys: Res<ButtonInput<KeyCode>>,
    player_velocity: Single<&mut Velocity, With<Player>>,
    mut walk_cycle_timer: ResMut<WalkCycleTimer>,
) {
    let mut velocity = player_velocity.into_inner();

    velocity.value = GridCoords {
        ..Default::default()
    };

    // walk_cycle_timer.timer.pause();

    if keys.pressed(KeyCode::KeyA) {
        walk_cycle_timer.timer.unpause();

        velocity.value = GridCoords {
            x: -1,
            ..Default::default()
        }
    } else if keys.pressed(KeyCode::KeyD) {
        walk_cycle_timer.timer.unpause();

        velocity.value = GridCoords {
            x: 1,
            ..Default::default()
        }
    } else if keys.pressed(KeyCode::KeyW) {
        walk_cycle_timer.timer.unpause();

        velocity.value = GridCoords {
            y: 1,
            ..Default::default()
        }
    } else if keys.pressed(KeyCode::KeyS) {
        walk_cycle_timer.timer.unpause();

        velocity.value = GridCoords {
            y: -1,
            ..Default::default()
        }
    }
}

fn move_player(
    mut query: Query<(&mut GridCoords, &Velocity), With<Player>>,
    mut walk_cycle_timer: ResMut<WalkCycleTimer>,
    time: Res<Time>,
) {
    for (mut player_grid_coords, velocity) in query.iter_mut() {
        if walk_cycle_timer.timer.remaining_secs() == TICK_DELTA
            && !walk_cycle_timer.timer.is_paused()
        {
            let destination = *player_grid_coords + velocity.value;
            *player_grid_coords = destination;
        } else if walk_cycle_timer.timer.remaining_secs() < JITTER_THRESHOLD {
            // } else if walk_cycle_timer.timer.just_finished() {
            walk_cycle_timer.timer.reset();
            walk_cycle_timer.timer.pause();
        }

        walk_cycle_timer.timer.tick(time.delta());
    }
}

fn translate_grid_coords_entities(
    mut commands: Commands,
    mut grid_coords_entities: Query<(Entity, &Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (entity, transform, grid_coords) in grid_coords_entities.iter_mut() {
        let destination = bevy_ecs_ldtk::utils::grid_coords_to_translation(
            *grid_coords,
            IVec2::splat(GRID_SIZE.into()),
        )
        .extend(transform.translation.z);

        let tween = Tween::new(
            EaseFunction::Linear,
            Duration::from_secs_f32(TICK_DELTA + JITTER_THRESHOLD),
            lens::TransformPositionLens {
                start: transform.translation,
                end: destination,
            },
        );

        commands.entity(entity).insert(TweenAnim::new(tween));
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
