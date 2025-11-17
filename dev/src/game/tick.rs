use std::time::Duration;

use bevy::prelude::*;

pub const TICK_DELTA: f32 = 0.25;

#[derive(Resource)]
pub struct MainTick {
    pub timer: Timer,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_main_tick);
    app.add_systems(Update, tick_timer);
}

fn setup_main_tick(mut commands: Commands) {
    commands.insert_resource(MainTick {
        timer: Timer::new(Duration::from_secs_f32(TICK_DELTA), TimerMode::Repeating),
    })
}

fn tick_timer(time: Res<Time>, mut config: ResMut<MainTick>) {
    config.timer.tick(time.delta());
}
