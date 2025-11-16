use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Default, Component)]
struct Player;

#[derive(Default, Component)]
struct Velocity {
    value: Vec2,
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (process_player, keyboard_event, accelerate_player).chain(),
    );
}

fn process_player(
    mut commands: Commands,
    new_entity_instances: Query<(Entity, &EntityInstance), Added<EntityInstance>>,
    server: Res<AssetServer>,
) {
    //     for (entity, entity_instance) in new_entity_instances.iter() {
    //         if entity_instance.identifier == "Player".to_string() {
    //             commands.entity(entity).insert((
    //                 Player,
    //                 AseSlice {
    //                     name: "player_idle".into(),
    //                     aseprite: server.load("textures/player/player.aseprite"),
    //                 },
    //                 Sprite::default(),
    //                 Velocity {
    //                     ..Default::default()
    //                 },
    //             ));
    //         }
    //     }
}

fn keyboard_event(
    keys: Res<ButtonInput<KeyCode>>,
    player_velocity: Single<&mut Velocity, With<Player>>,
) {
    let mut velocity = player_velocity.into_inner();

    velocity.value = Vec2 {
        ..Default::default()
    };

    if keys.pressed(KeyCode::KeyA) {
        velocity.value += Vec2 {
            x: -3000.0,
            ..Default::default()
        }
    }

    if keys.pressed(KeyCode::KeyD) {
        velocity.value += Vec2 {
            x: 3000.0,
            ..Default::default()
        }
    }

    if keys.just_pressed(KeyCode::Space) {
        velocity.value += Vec2 {
            y: 3000.0,
            ..Default::default()
        }
    }
}

fn accelerate_player(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Player>>) {
    //     let delta_secs = time.delta_secs();

    //     for (mut linear_velocity, mut transform, velocity) in &mut query {
    //         transform.rotation = Quat::from_rotation_z((0.0_f32).to_radians());
    //         linear_velocity.0.x = velocity.value.x * delta_secs;
    //         linear_velocity.0.y += velocity.value.y * delta_secs;
    //     }
}
