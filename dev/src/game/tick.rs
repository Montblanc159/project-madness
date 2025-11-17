use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct MainTick {
    /// How often to spawn a new bomb? (repeating timer)
    pub timer: Timer,
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_main_tick);
    app.add_systems(Update, tick_timer);
}

fn setup_main_tick(mut commands: Commands) {
    commands.insert_resource(MainTick {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs_f32(0.25), TimerMode::Repeating),
    })
}

fn tick_timer(time: Res<Time>, mut config: ResMut<MainTick>) {
    // tick the timer
    config.timer.tick(time.delta());
}
