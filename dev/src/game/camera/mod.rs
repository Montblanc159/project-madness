use bevy::prelude::*;

use crate::game::camera::post_processing_shaders::level_transition_shader::{
    self, LevelTransitionShaderSettings,
};

mod post_processing_shaders;

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

#[derive(Component)]
#[require(Transform)]
pub struct CameraTarget;

pub fn plugin(app: &mut App) {
    app.add_plugins(level_transition_shader::plugin);
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Update, lock_camera_on_target);
}

pub fn spawn_camera(mut commands: Commands, camera_target: Query<&Transform, With<CameraTarget>>) {
    let target_transform = *camera_target.single().unwrap_or(&Transform {
        ..Default::default()
    });

    commands.spawn((
        MainCamera,
        IsDefaultUiCamera,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgba(0., 0., 0., 1.)),
            ..Default::default()
        },
        Projection::Orthographic(OrthographicProjection {
            scale: 0.1,
            ..OrthographicProjection::default_2d()
        }),
        #[allow(clippy::needless_update)]
        LevelTransitionShaderSettings {
            time: 0.0,
            ..default()
        },
        Transform::from_translation(target_transform.translation),
    ));
}

fn lock_camera_on_target(
    camera: Single<(Entity, &mut Transform), With<MainCamera>>,
    target: Single<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
) {
    let (_camera_entity, mut camera_transform) = camera.into_inner();
    let target = target.into_inner();

    camera_transform.translation = target.translation;
}
