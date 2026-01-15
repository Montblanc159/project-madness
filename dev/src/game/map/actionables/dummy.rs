use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;

use crate::game::map::{GRID_SIZE, utils};

const IDENTIFIER: &str = "DummyAction";

#[derive(Component)]
struct DummyAction;

impl super::Action for DummyAction {
    fn activate(&self) {
        println!("Dummy action activated");
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            super::despawn_entity_instances::<DummyAction>,
            spawn_entity_instance,
            super::activate::<DummyAction>,
        ),
    );
}

fn spawn_entity_instance(
    mut commands: Commands,
    new_entity_instances: Query<(&EntityInstance, &Transform), Added<EntityInstance>>,
) {
    for (entity_instance, transform) in new_entity_instances.iter() {
        if entity_instance.identifier == IDENTIFIER {
            let entity_origin = utils::entity_top_left_pixel_position(
                transform.translation,
                entity_instance.width,
                entity_instance.height,
                GRID_SIZE,
            );

            let origin_grid_coords = bevy_ecs_ldtk::utils::translation_to_grid_coords(
                entity_origin,
                IVec2::splat(GRID_SIZE),
            );

            let full_span_grid_coords = utils::grid_coords_from_entity_size(
                origin_grid_coords,
                entity_instance.width,
                entity_instance.height,
                GRID_SIZE,
            );

            for grid_coords in full_span_grid_coords {
                let translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
                    grid_coords,
                    IVec2::splat(GRID_SIZE),
                )
                .extend(transform.translation.z);

                commands.spawn((
                    Transform {
                        translation,
                        scale: Vec3 {
                            x: 1.,
                            y: 1.,
                            z: 1.,
                        },
                        ..*transform
                    },
                    DummyAction,
                    grid_coords,
                ));
            }
        }
    }
}
