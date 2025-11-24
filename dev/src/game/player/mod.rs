use std::time::Duration;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tweening::*;

use super::camera::CameraTarget;
use super::map::colliders::{GRID_SIZE, LevelColliders};
use super::tick::TICK_DELTA;

pub const JITTER_THRESHOLD: f32 = 0.015;

#[derive(Resource)]
pub struct WalkCycleTimer {
    pub timer: Timer,
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component, Debug)]
struct Velocity {
    value: GridCoords,
}

#[derive(Message)]
pub struct Teleported {
    pub entity: Entity,
    pub grid_coords: GridCoords,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init_walk_cycle_timer);
    app.add_message::<Teleported>();
    app.add_systems(
        Update,
        (
            spawn_player,
            teleport_player,
            player_movement_input,
            update_player_grid_coords,
            set_translate_with_grid_coords,
        )
            .chain(),
    );
}

fn spawn_player(
    mut commands: Commands,
    new_entity_instances: Query<&EntityInstance, Added<EntityInstance>>,
    players: Query<Entity, With<Player>>,
    server: Res<AssetServer>,
) {
    for entity_instance in new_entity_instances.iter() {
        if entity_instance.identifier == "Player".to_string() && !players.iter().next().is_some() {
            println!("Spawning player");
            commands.spawn((
                Player,
                CameraTarget,
                AseSlice {
                    name: "player_idle".into(),
                    aseprite: server.load("textures/player/player.aseprite"),
                },
                Sprite::default(),
                GridCoords {
                    ..entity_instance.grid.into()
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
    player_velocities: Query<&mut Velocity, With<Player>>,
    mut walk_cycle_timer: ResMut<WalkCycleTimer>,
) {
    for mut velocity in player_velocities {
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
}

fn update_player_grid_coords(
    mut query: Query<(&mut GridCoords, &Velocity), With<Player>>,
    mut walk_cycle_timer: ResMut<WalkCycleTimer>,
    level_colliders: Res<LevelColliders>,
    time: Res<Time>,
) {
    for (mut player_grid_coords, velocity) in query.iter_mut() {
        let destination = *player_grid_coords + velocity.value;

        if walk_cycle_timer.timer.remaining_secs() == TICK_DELTA
            && !walk_cycle_timer.timer.is_paused()
            && !level_colliders.in_collider(&destination)
        {
            *player_grid_coords = destination;
        } else if walk_cycle_timer.timer.remaining_secs() <= JITTER_THRESHOLD {
            walk_cycle_timer.timer.reset();
            walk_cycle_timer.timer.pause();
        }

        walk_cycle_timer.timer.tick(time.delta());
    }
}

fn teleport_player(
    mut teleport_event: MessageReader<Teleported>,
    mut query: Query<(&mut GridCoords, &mut Velocity, &mut Transform), With<Player>>,
    mut walk_cycle_timer: ResMut<WalkCycleTimer>,
) {
    for event in teleport_event.read() {
        if let Ok((mut player_grid_coords, mut player_velocity, mut player_transform)) =
            query.get_mut(event.entity)
        {
            player_velocity.value = GridCoords {
                ..Default::default()
            };

            player_transform.translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
                event.grid_coords,
                IVec2::splat(GRID_SIZE.into()),
            )
            .extend(player_transform.translation.z);

            walk_cycle_timer.timer.reset();
            walk_cycle_timer.timer.pause();

            *player_grid_coords = event.grid_coords;
        }
    }
}

fn set_translate_with_grid_coords(
    mut commands: Commands,
    mut grid_coords_entities: Query<
        (Entity, &Transform, &GridCoords),
        (Changed<GridCoords>, With<Player>),
    >,
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
