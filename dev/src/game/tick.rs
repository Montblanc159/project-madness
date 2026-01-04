use std::time::Duration;

use bevy::prelude::*;

const DEFAULT_GAME_BEAT_DIVISION: f32 = 0.5;
const DEFAULT_BPM: f32 = 120.;

#[derive(Resource)]
pub struct MainTick {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct MainTickCounter {
    pub value: u8,
}

#[derive(Resource)]
pub struct TickDelta(pub f32);

#[derive(Resource, Clone)]
pub struct GameTempo {
    bpm: f32,
    division: f32,
}

impl Into<TickDelta> for GameTempo {
    fn into(self) -> TickDelta {
        TickDelta(60. / (self.bpm / self.division))
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(MainTickCounter { value: 1 });

    let game_tempo = GameTempo {
        bpm: DEFAULT_BPM,
        division: DEFAULT_GAME_BEAT_DIVISION,
    };

    app.insert_resource::<TickDelta>(game_tempo.clone().into());
    app.insert_resource(game_tempo);

    app.add_systems(Startup, setup_main_tick);
    app.add_systems(
        Update,
        (
            update_tick_delta,
            update_main_tick,
            tick_timer,
            count_timer_repeats,
        )
            .chain(),
    );
}

fn setup_main_tick(mut commands: Commands, tick_delta: Res<TickDelta>) {
    commands.insert_resource(MainTick {
        timer: Timer::new(Duration::from_secs_f32(tick_delta.0), TimerMode::Repeating),
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

fn update_tick_delta(mut tick_delta: ResMut<TickDelta>, game_tempo: Res<GameTempo>) {
    if game_tempo.is_changed() {
        *tick_delta = game_tempo.clone().into();
    }
}

fn update_main_tick(mut main_tick: ResMut<MainTick>, tick_delta: Res<TickDelta>) {
    if tick_delta.is_changed() {
        main_tick
            .timer
            .set_duration(Duration::from_secs_f32(tick_delta.0));
    }
}
