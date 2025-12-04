use std::time::Duration;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_tweening::*;

use super::camera::CameraTarget;
use super::map::{GRID_SIZE, colliders::LevelColliders};
use super::tick::TICK_DELTA;

pub const JITTER_THRESHOLD: f32 = 0.015;
const ACTION_Z_DEPTH: f32 = 2.;
const PLAYER_Z_DEPTH: f32 = 2.;

#[derive(Resource)]
pub struct WalkCycleTimer {
    pub timer: Timer,
}

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Component, Debug)]
struct Velocity {
    value: IVec2,
}

#[derive(Component, Debug, Clone)]
struct ActionZone {
    value: IVec2,
    display: Option<Entity>,
}

#[derive(Component, Debug)]
struct ActionZoneDisplay;

#[derive(Component)]
enum Facing {
    North,
    East,
    South,
    West,
}

#[derive(Message)]
pub struct Teleported {
    pub entity: Entity,
    pub grid_coords: IVec2,
}

#[derive(Message)]
pub struct Activate {
    pub _entity: Entity,
    pub grid_coords: IVec2,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, init_walk_cycle_timer);
    app.add_message::<Teleported>();
    app.add_message::<Activate>();
    app.add_systems(
        Update,
        (
            display_action_zone,
            spawn_player,
            set_action_grid_coords,
            activate,
            teleport_player,
            player_movement_input,
            update_player_grid_coords,
            set_translate_with_grid_coords,
            update_display_action_zone,
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
                ActionZone {
                    value: (entity_instance.grid + ivec2(0, -1)).into(),
                    display: None,
                },
                Facing::South,
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
    player_velocities: Query<(&mut Velocity, &mut Facing), With<Player>>,
    mut walk_cycle_timer: ResMut<WalkCycleTimer>,
) {
    for (mut velocity, mut facing) in player_velocities {
        velocity.value = IVec2 {
            ..Default::default()
        };

        if keys.pressed(KeyCode::KeyA) {
            walk_cycle_timer.timer.unpause();

            *facing = Facing::West;
            velocity.value = IVec2 {
                x: -1,
                ..Default::default()
            }
        } else if keys.pressed(KeyCode::KeyD) {
            walk_cycle_timer.timer.unpause();

            *facing = Facing::East;
            velocity.value = IVec2 {
                x: 1,
                ..Default::default()
            }
        } else if keys.pressed(KeyCode::KeyW) {
            walk_cycle_timer.timer.unpause();

            *facing = Facing::North;
            velocity.value = IVec2 {
                y: 1,
                ..Default::default()
            }
        } else if keys.pressed(KeyCode::KeyS) {
            walk_cycle_timer.timer.unpause();

            *facing = Facing::South;
            velocity.value = IVec2 {
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
        let destination = *player_grid_coords + velocity.value.into();

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
            player_velocity.value = IVec2 {
                ..Default::default()
            };

            player_transform.translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
                event.grid_coords.into(),
                IVec2::splat(GRID_SIZE.into()),
            )
            .extend(PLAYER_Z_DEPTH);

            walk_cycle_timer.timer.reset();
            walk_cycle_timer.timer.pause();

            *player_grid_coords = event.grid_coords.into();
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
        .extend(PLAYER_Z_DEPTH);

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

fn set_action_grid_coords(
    players: Query<(&mut ActionZone, &GridCoords, &Facing), (With<Player>, Changed<Facing>)>,
) {
    for (mut action_zone, grid_coords, facing) in players {
        action_zone.value = (*grid_coords
            + (match facing {
                Facing::North => GridCoords { y: 1, x: 0 },
                Facing::East => GridCoords { y: 0, x: 1 },
                Facing::South => GridCoords { y: -1, x: 0 },
                Facing::West => GridCoords { y: 0, x: -1 },
            }))
        .into()
    }
}

fn display_action_zone(
    action_zones: Query<&mut ActionZone, (With<Player>, Added<ActionZone>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::Srgba(Srgba {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 0.05,
    });

    for mut action_zone in action_zones {
        let zone = action_zone.value;

        let display = commands
            .spawn((
                Mesh2d(meshes.add(Rectangle::new(2., 2.))),
                MeshMaterial2d(materials.add(color)),
                ActionZoneDisplay,
                Transform {
                    translation: bevy_ecs_ldtk::utils::grid_coords_to_translation(
                        zone.into(),
                        IVec2::splat(GRID_SIZE.into()),
                    )
                    .extend(ACTION_Z_DEPTH),
                    ..Default::default()
                },
            ))
            .id();

        action_zone.display = Some(display);
    }
}

fn update_display_action_zone(
    action_zones: Query<&ActionZone, (Changed<ActionZone>, With<Player>)>,
    mut commands: Commands,
) {
    for action_zone in action_zones {
        if let Some(entity) = action_zone.display {
            commands
                .entity(entity)
                .remove::<Transform>()
                .insert(Transform {
                    translation: bevy_ecs_ldtk::utils::grid_coords_to_translation(
                        action_zone.value.into(),
                        IVec2::splat(GRID_SIZE.into()),
                    )
                    .extend(ACTION_Z_DEPTH),
                    ..Default::default()
                });
        }
    }
}

fn activate(
    keys: Res<ButtonInput<KeyCode>>,
    players: Query<(Entity, &ActionZone), With<Player>>,
    mut activate_event: MessageWriter<Activate>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (entity, action_zone) in players {
            activate_event.write(Activate {
                _entity: entity,
                grid_coords: action_zone.value.into(),
            });
        }
    }
}
