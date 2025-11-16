use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((LdtkPlugin, AsepriteUltraPlugin));
}
