use bevy::prelude::*;
use bevy_firefly::prelude::*;

use crate::game::{
    camera::{MainCamera, spawn_camera},
    map::{GRID_SIZE, colliders::Wall},
};

const DEFAULT_INTENSITY: f32 = 1.;

pub mod torch;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, add_config_to_camera.after(spawn_camera));
    app.add_plugins(torch::plugin);
    app.add_systems(Update, add_occluder::<Wall>);
}

fn add_config_to_camera(mut commands: Commands, camera: Single<Entity, With<MainCamera>>) {
    let camera = camera.into_inner();

    commands.entity(camera).insert(FireflyConfig {
        // ambient_brightness: 0.2,
        softness: None,
        light_bands: Some(16),
        ..default()
    });
}

fn add_occluder<T: Component>(mut commands: Commands, entities: Query<Entity, Added<T>>) {
    for entity in entities {
        commands.entity(entity).insert(Occluder2d::rectangle(
            GRID_SIZE as f32 + 0.01,
            GRID_SIZE as f32 + 0.01,
        ));
    }
}

trait LightParameters {
    fn color() -> Color {
        Color::srgb(1., 1., 1.)
    }
    fn intensity() -> f32 {
        DEFAULT_INTENSITY
    }
    fn range() -> f32 {
        GRID_SIZE as f32 * 3.
    }
    fn offset() -> Vec3 {
        Vec3::ZERO
    }
}

fn add_lights<T: Component + LightParameters>(
    mut commands: Commands,
    entities: Query<Entity, Added<T>>,
) {
    for entity in entities {
        commands.entity(entity).insert(PointLight2d {
            color: T::color(),
            intensity: T::intensity(),
            range: T::range(),
            offset: T::offset(),
            ..Default::default()
        });
    }
}
