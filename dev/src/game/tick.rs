use std::time::Duration;

use bevy::prelude::*;

const DEFAULT_BPM: f32 = 120.;
const DEFAULT_BEATS_PER_MEASURE: f32 = 4.;
const DEFAULT_NOTES_PER_MEASURE: f32 = 4.;

#[derive(Resource)]
pub struct MainTick {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct MainTickCounter {
    pub value: u8,
}

/// Stores the duration of the different divisions rhythming the game
#[derive(Resource)]
pub struct TickDelta {
    /// Returns the duration of a beat
    pub _beat: f32,
    /// Returns the duration of a measure
    pub measure: f32,
    /// Returns the duration of a note
    pub note: f32,
}

#[derive(Resource, Clone)]
pub struct GameTempo {
    pub bpm: f32,
    pub beats_per_measure: f32,
    pub notes_per_measure: f32,
}

impl From<GameTempo> for TickDelta {
    fn from(val: GameTempo) -> Self {
        TickDelta {
            _beat: 60. / val.bpm,
            measure: (60. / val.bpm) * val.beats_per_measure,
            note: ((60. / val.bpm) * val.beats_per_measure) / val.notes_per_measure,
        }
    }
}

pub fn plugin(app: &mut App) {
    app.insert_resource(MainTickCounter { value: 1 });

    let game_tempo = GameTempo {
        bpm: DEFAULT_BPM,
        beats_per_measure: DEFAULT_BEATS_PER_MEASURE,
        notes_per_measure: DEFAULT_NOTES_PER_MEASURE,
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
        timer: Timer::new(
            Duration::from_secs_f32(tick_delta.note),
            TimerMode::Repeating,
        ),
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
            .set_duration(Duration::from_secs_f32(tick_delta.note));
    }
}
