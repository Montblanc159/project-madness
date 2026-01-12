use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_tweening::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(LdtkPlugin);
    app.add_plugins(AsepriteUltraPlugin);
    app.add_plugins(TweeningPlugin);
    app.add_plugins(AudioPlugin);
    app.add_plugins(SpatialAudioPlugin);
}
