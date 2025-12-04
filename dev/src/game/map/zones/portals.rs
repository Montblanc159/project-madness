use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::{
    map::{CurrentLevelInfos, GRID_SIZE, utils, zones::Zones},
    player::{Player, Teleported},
};

#[derive(Component, Default, Clone, Debug)]
struct Portal {
    to: String,
}

#[derive(Component)]
struct NotTeleportable;

impl Zones<Portal> {
    pub fn portal_on_gridcoord(&self, grid_coords: &GridCoords) -> Option<&Portal> {
        self.locations.get(grid_coords)
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Zones::<Portal> {
        ..Default::default()
    });

    app.add_systems(
        Update,
        (
            super::empty_zones_cache::<Portal>,
            super::remove_zones::<Portal>,
            spawn_portals,
            super::cache_zones::<Portal>,
            spawn_player_on_portal,
            activate,
            remove_not_teleportable,
        )
            .chain(),
    );
}

fn spawn_portals(
    mut commands: Commands,
    new_entity_instances: Query<(&EntityInstance, &Transform), Added<EntityInstance>>,
) {
    for (entity_instance, transform) in new_entity_instances.iter() {
        if &entity_instance.identifier == &"Portal".to_string() {
            let fields = utils::get_fields(entity_instance, vec!["To"]);

            if let Some(to_field) = fields.strings.get("To") {
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
                            scale: Vec3 {
                                x: 1.,
                                y: 1.,
                                z: 1.,
                            },
                            translation: translation,
                            ..*transform
                        },
                        Portal {
                            to: to_field.clone(),
                        },
                        grid_coords,
                    ));
                }
            }
        }
    }
}

fn activate(
    mut commands: Commands,
    portals: Res<Zones<Portal>>,
    players: Query<
        (Entity, &GridCoords),
        (With<Player>, Changed<GridCoords>, Without<NotTeleportable>),
    >,
    mut level_selection: ResMut<LevelSelection>,
) {
    for (entity, grid_coords) in players {
        if portals.activated(grid_coords) {
            if let Some(portal) = portals.portal_on_gridcoord(grid_coords) {
                *level_selection = LevelSelection::Identifier(portal.to.clone());
                commands.entity(entity).insert(NotTeleportable);
            }
        }
    }
}

fn spawn_player_on_portal(
    level_portals: ResMut<Zones<Portal>>,
    mut level_messages: MessageReader<LevelEvent>,
    mut teleport_message: MessageWriter<Teleported>,
    players: Query<Entity, With<Player>>,
    level_infos: Res<CurrentLevelInfos>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(_) = level_event {
            for player in players {
                for (grid_coords, portal) in level_portals.locations.iter() {
                    if let Some(coming_from) = &level_infos.coming_from {
                        if portal.to == *coming_from {
                            teleport_message.write(Teleported {
                                entity: player,
                                grid_coords: (*grid_coords).into(),
                            });

                            break;
                        }
                    }
                }
            }
        }
    }
}

fn remove_not_teleportable(
    mut commands: Commands,
    portals: Res<Zones<Portal>>,
    players: Query<(Entity, &GridCoords), (With<NotTeleportable>, Changed<GridCoords>)>,
) {
    for (entity, grid_coords) in players {
        if !portals.activated(grid_coords) {
            commands.entity(entity).remove::<NotTeleportable>();
        }
    }
}
