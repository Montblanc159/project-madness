use std::time::Duration;

use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
};
use bevy_ecs_ldtk::LevelEvent;

const SHADER_ASSET_PATH: &str = "shaders/post_processing/level_transition.wgsl";

#[derive(Resource)]
struct TransitionTimer {
    value: Timer,
}

// This is the component that will get passed to the shader
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct LevelTransitionShaderSettings {
    pub time: f32,
    // WebGL2 structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    _webgl2_padding: Vec3,
}

impl super::ShaderAsset for LevelTransitionShaderSettings {
    fn shader_asset_path() -> String {
        SHADER_ASSET_PATH.into()
    }
}

pub fn plugin(app: &mut App) {
    app.add_plugins(super::plugin::<LevelTransitionShaderSettings>);
    app.insert_resource(TransitionTimer {
        value: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
    });
    app.add_systems(Update, update_settings);
}

fn update_settings(
    mut settings: Query<&mut LevelTransitionShaderSettings>,
    mut timer: ResMut<TransitionTimer>,
    time: Res<Time>,
    mut level_event: MessageReader<LevelEvent>,
) {
    for event in level_event.read() {
        if let LevelEvent::Despawned(_) = event {
            timer.value.reset();
        }
    }

    if !timer.value.just_finished() {
        for mut setting in &mut settings {
            // This will then be extracted to the render world and uploaded to the GPU automatically by the [`UniformComponentPlugin`]
            setting.time = timer.value.elapsed_secs();
            timer.value.tick(time.delta());
        }
    }
}
