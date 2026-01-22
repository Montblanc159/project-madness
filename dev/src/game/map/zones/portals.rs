use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::{
    global::{GameState, despawn_entity_on_level_change},
    map::{ChangeLevel, CurrentLevelInfos, utils, zones::Zones},
    player::{Player, Teleported},
};

const IDENTIFIER: &str = "Portal";
const FIELDS: [&str; 1] = ["To"];

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

impl super::Zone for Portal {
    fn identifier() -> String {
        IDENTIFIER.into()
    }

    fn new(entity_instance: &EntityInstance) -> impl Bundle {
        let fields = utils::get_fields(entity_instance, FIELDS.to_vec());

        if let Some(to_field) = fields.strings.get("To") {
            Portal {
                to: to_field.into(),
            }
        } else {
            panic!("To field not found on entity instance")
        }
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
            despawn_entity_on_level_change::<Portal>,
            super::spawn_zones::<Portal>,
            super::cache_zones::<Portal>,
            spawn_player_on_portal,
            activate,
            remove_not_teleportable,
        )
            .run_if(in_state(GameState::InGame))
            .chain(),
    );
}

fn activate(
    mut commands: Commands,
    mut event: MessageWriter<ChangeLevel>,
    portals: Res<Zones<Portal>>,
    players: Query<
        (Entity, &GridCoords),
        (With<Player>, Changed<GridCoords>, Without<NotTeleportable>),
    >,
) {
    for (entity, grid_coords) in players {
        if portals.activated(grid_coords)
            && let Some(portal) = portals.portal_on_gridcoord(grid_coords)
        {
            event.write(ChangeLevel {
                identifier: portal.to.clone(),
            });

            commands.entity(entity).insert(NotTeleportable);
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
                    if let Some(coming_from) = &level_infos.coming_from
                        && portal.to == *coming_from
                    {
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
