use std::time::Duration;

use bevy::prelude::*;

pub const TICK_DELTA: f32 = 0.25;

#[derive(Resource)]
pub struct MainTick {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct MainTickCounter {
    pub value: u8,
}

pub fn plugin(app: &mut App) {
    app.insert_resource(MainTickCounter { value: 1 });
    app.add_systems(Startup, setup_main_tick);
    app.add_systems(Update, (tick_timer, count_timer_repeats).chain());
}

fn setup_main_tick(mut commands: Commands) {
    commands.insert_resource(MainTick {
        timer: Timer::new(Duration::from_secs_f32(TICK_DELTA), TimerMode::Repeating),
    })
}

fn tick_timer(time: Res<Time>, mut config: ResMut<MainTick>) {
    config.timer.tick(time.delta());
}

fn count_timer_repeats(main_tick: Res<MainTick>, mut counter: ResMut<MainTickCounter>) {
    if main_tick.timer.just_finished() {
        if counter.value == 120 {
            counter.value = 1;
        } else {
            counter.value += 1;
        }
    }
}
