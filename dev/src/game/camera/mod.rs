use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::*;

use super::player::JITTER_THRESHOLD;
use super::tick::TICK_DELTA;

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

#[derive(Component)]
#[require(Transform)]
pub struct CameraTarget;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera);
    app.add_systems(Update, lock_camera_on_target);
}

fn initialize_camera(mut commands: Commands, camera_target: Query<&Transform, With<CameraTarget>>) {
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
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 160.,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_translation(target_transform.translation),
    ));
}

fn lock_camera_on_target(
    mut commands: Commands,
    camera: Single<(Entity, &Transform), With<MainCamera>>,
    target: Single<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
) {
    let (camera_entity, camera_transform) = camera.into_inner();
    let target = target.into_inner();

    let tween = Tween::new(
        EaseFunction::Linear,
        Duration::from_secs_f32(TICK_DELTA + JITTER_THRESHOLD),
        lens::TransformPositionLens {
            start: camera_transform.translation,
            end: target.translation,
        },
    );

    commands.entity(camera_entity).insert(TweenAnim::new(tween));
}
