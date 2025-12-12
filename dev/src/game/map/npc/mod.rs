use std::time::Duration;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::AseSlice;
use bevy_ecs_ldtk::{EntityInstance, GridCoords, LevelEvent};
use bevy_tweening::*;
use rand::prelude::*;

use crate::game::{
    map::{
        GRID_SIZE,
        colliders::{Collider, LevelColliders},
        zones::{Zones, wander_zones::WanderZone},
    },
    player::{Activate, JITTER_THRESHOLD},
    tick::{MainTick, MainTickCounter, TICK_DELTA},
    ui::dialogs::DialogEvent,
};

mod dummy_npc;

const NPC_Z_DEPTH: f32 = 2.;

trait Npc {
    fn identifier() -> String;
    fn aseslice(server: &Res<AssetServer>) -> AseSlice;
    fn new() -> impl Bundle;
}

#[derive(Component)]
struct Wanderer;

#[derive(Component)]
struct Talkable;

// #[derive(Component)]
// struct DialogFilePath(String);

pub fn plugin(app: &mut App) {
    app.add_plugins(dummy_npc::plugin);
    app.add_systems(Update, (wander, talk));
}

fn spawn_npc<T: Component + Npc>(
    mut commands: Commands,
    new_entity_instances: Query<&EntityInstance, Added<EntityInstance>>,
    server: Res<AssetServer>,
) {
    for entity_instance in new_entity_instances.iter() {
        if entity_instance.identifier == T::identifier() {
            commands.spawn((
                T::aseslice(&server),
                Sprite::default(),
                GridCoords {
                    ..entity_instance.grid.into()
                },
                Collider,
                Transform {
                    translation: bevy_ecs_ldtk::utils::grid_coords_to_translation(
                        entity_instance.grid.into(),
                        IVec2::splat(GRID_SIZE),
                    )
                    .extend(NPC_Z_DEPTH),
                    ..Default::default()
                },
                T::new(),
            ));
        }
    }
}

fn despawn_npc<T: Component + Npc>(
    npcs: Query<Entity, With<T>>,
    mut commands: Commands,
    mut level_messages: MessageReader<LevelEvent>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Despawned(_) = level_event {
            for entity in npcs {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn wander(
    npc: Query<&mut GridCoords, With<Wanderer>>,
    level_colliders: Res<LevelColliders>,
    main_tick: Res<MainTick>,
    main_tick_counter: Res<MainTickCounter>,
    wandering_zones: Res<Zones<WanderZone>>,
) {
    if main_tick.timer.just_finished() && main_tick_counter.value % 4 == 0 {
        let mut rng = rand::rng();
        let nums: Vec<i32> = (0..2).collect();

        for mut grid_coords in npc {
            let move_distance = nums.choose(&mut rng);
            let left_or_up = rand::random_bool(1.0 / 2.0);
            let add_or_sub = rand::random_bool(1.0 / 2.0);

            if let Some(move_distance) = move_distance {
                let movement_vector = if left_or_up {
                    IVec2 {
                        x: *move_distance,
                        y: 0,
                    }
                } else {
                    IVec2 {
                        x: 0,
                        y: *move_distance,
                    }
                };

                let destination = if add_or_sub {
                    *grid_coords + movement_vector.into()
                } else {
                    *grid_coords - movement_vector.into()
                };

                if !level_colliders.in_collider(&destination)
                    && wandering_zones.activated(&destination)
                {
                    *grid_coords = destination;
                }
            }
        }
    }
}

fn update_npc_position<T: Component + Npc>(
    mut commands: Commands,
    npc: Query<(Entity, &Transform, &GridCoords), (With<T>, Changed<GridCoords>)>,
) {
    for (entity, transform, grid_coords) in npc {
        let destination = bevy_ecs_ldtk::utils::grid_coords_to_translation(
            *grid_coords,
            IVec2::splat(GRID_SIZE.into()),
        )
        .extend(NPC_Z_DEPTH);

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

fn talk(
    mut activate_event: MessageReader<Activate>,
    // asset_server: Res<AssetServer>,
    mut dialog_event: MessageWriter<DialogEvent>,
    talkable_npc: Query<(&GridCoords), With<Talkable>>,
) {
    for event in activate_event.read() {
        for grid_coords in talkable_npc {
            if event.grid_coords == (*grid_coords).into() {
                dialog_event.write(DialogEvent {
                    source: "foo".into(),
                    image: "foobar".into(),
                    body: "bar".into(),
                });
            }
        }
    }
}
