use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::game::{
    audio::music::PlaySong,
    global::{GameState, despawn_entity_on_level_change},
    map::{utils, zones::Zones},
    player::Player,
};

const IDENTIFIER: &str = "MusicZone";
const FIELDS: [&str; 2] = ["SongTitle", "Part"];

#[derive(Component, Default, Clone, Debug)]
pub struct MusicZone {
    song_title: String,
    part: String,
}

impl Zones<MusicZone> {
    pub fn zone_on_gridcoord(&self, grid_coords: &GridCoords) -> Option<&MusicZone> {
        self.locations.get(grid_coords)
    }
}

impl super::Zone for MusicZone {
    fn identifier() -> String {
        IDENTIFIER.into()
    }

    fn new(entity_instance: &EntityInstance) -> impl Bundle {
        let fields = utils::get_fields(entity_instance, FIELDS.to_vec());

        let mut zone = MusicZone {
            ..Default::default()
        };

        if let Some(song_title) = fields.strings.get("SongTitle") {
            zone.song_title = song_title.clone();
        } else {
            panic!("Song title field not found on entity instance")
        }

        if let Some(part) = fields.strings.get("Part") {
            zone.part = part.clone();
        } else {
            panic!("Part field not found on entity instance")
        }

        zone
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(Zones::<MusicZone> {
        ..Default::default()
    });

    app.add_systems(
        Update,
        (
            super::empty_zones_cache::<MusicZone>,
            despawn_entity_on_level_change::<MusicZone>,
            super::spawn_zones::<MusicZone>,
            super::cache_zones::<MusicZone>,
            activate,
        )
            .chain()
            .run_if(in_state(GameState::InGame)),
    );
}

fn activate(
    zones: Res<Zones<MusicZone>>,
    players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
    mut event: MessageWriter<PlaySong>,
) {
    for grid_coords in players {
        if zones.activated(grid_coords)
            && let Some(zone) = zones.zone_on_gridcoord(grid_coords)
        {
            event.write(PlaySong {
                song_title: zone.song_title.clone(),
                part: zone.part.clone(),
            });
        }
    }
}
