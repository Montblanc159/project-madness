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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawns_camera() {
        // Setup
        let mut app = App::new();
        app.add_systems(Startup, spawn_camera);

        // Run
        app.update();
        app.update();

        // Check
        let main_camera = app.world_mut().query::<&MainCamera>().single(app.world());
        assert!(main_camera.is_ok(), "There should be only one MainCamera.")
    }

    #[test]
    fn sets_transform_to_default_without_target() {
        // Setup
        let mut app = App::new();
        app.add_systems(Startup, spawn_camera);

        // Run
        app.update();

        // Check
        let main_camera_transform = app
            .world_mut()
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(app.world());

        assert!(
            main_camera_transform.is_ok(),
            "There should be only one MainCamera."
        );

        assert_eq!(
            main_camera_transform.unwrap().translation,
            Transform { ..default() }.translation
        );
    }

    #[test]
    fn sets_transform_to_target() {
        // Setup
        let mut app = App::new();
        let translation = vec3(5., 5., 5.);
        app.world_mut()
            .spawn((CameraTarget, Transform::from_translation(translation)));
        app.add_systems(Startup, spawn_camera);

        // Run
        app.update();

        // Check
        let main_camera_transform = app
            .world_mut()
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(app.world());

        assert!(
            main_camera_transform.is_ok(),
            "There should be only one MainCamera."
        );

        assert_eq!(main_camera_transform.unwrap().translation, translation);
    }

    #[test]
    fn updates_transform_with_target() {
        // Setup
        let mut app = App::new();
        app.world_mut()
            .spawn((CameraTarget, Transform::from_translation(vec3(5., 5., 5.))));

        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, lock_camera_on_target);

        app.update();
        let target_transform = app
            .world_mut()
            .query_filtered::<&mut Transform, With<CameraTarget>>()
            .single_mut(app.world_mut());

        let target_translation = vec3(10., 10., 10.);

        target_transform.unwrap().translation = target_translation;

        // Run
        app.update();

        // Check
        let main_camera_transform = app
            .world_mut()
            .query_filtered::<&Transform, With<MainCamera>>()
            .single(app.world());

        assert!(
            main_camera_transform.is_ok(),
            "There should be only one MainCamera."
        );

        assert_eq!(
            main_camera_transform.unwrap().translation,
            target_translation
        );
    }
}
