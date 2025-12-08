use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::map::zones::Zones;

const IDENTIFIER: &str = "WanderZone";

#[derive(Component, Default, Clone, Debug)]
pub struct WanderZone;

impl super::Zone for WanderZone {
    fn identifier() -> String {
        IDENTIFIER.into()
    }

    fn new(_entity_instance: &EntityInstance) -> impl Bundle {
        WanderZone
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Zones::<WanderZone> {
        ..Default::default()
    });

    app.add_systems(
        Update,
        (
            super::empty_zones_cache::<WanderZone>,
            super::remove_zones::<WanderZone>,
            super::spawn_zones::<WanderZone>,
            super::cache_zones::<WanderZone>,
        )
            .chain(),
    );
}
