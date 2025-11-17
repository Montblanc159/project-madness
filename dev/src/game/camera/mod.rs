use bevy::prelude::*;

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

fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        IsDefaultUiCamera,
        Camera {
            // disable clearing completely (pixels stay as they are)
            // (preserves output from previous frame or camera/pass)
            clear_color: ClearColorConfig::Custom(Color::srgba(0., 0., 0., 1.)),
            ..Default::default()
        },
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: 160.,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(240.0 / 2.0, 160.0 / 2.0, 0.0),
    ));
}

fn lock_camera_on_target(
    camera: Single<&mut Transform, With<MainCamera>>,
    target: Single<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
) {
    let mut camera = camera.into_inner();
    let target = target.into_inner();

    camera.translation = target.translation;
}
