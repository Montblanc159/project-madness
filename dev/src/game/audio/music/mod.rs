use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioSource};

use crate::game::tick::{GameTempo, MainTick, MainTickCounter, TickDelta};

mod introduction;

/// Main song parameters, influencing game tempo
#[derive(Component)]
struct Song {
    bpm: f32,
    beats_per_measure: f32,
    notes_per_measure: f32,
}

/// This is meant to be added on a `Component` used to
/// identify which song is going to be queued
/// Stores all parameters linked to a song
/// A 8/5 rythmed song with 90 bpm called T Song would be implemented as follows :
/// ```
/// impl SongParameters for TSong {
///     fn title() -> String {
///         "T Song"
///     }
///
///     fn new() -> impl Bundle  {
///         Self
///     }
///
///     fn bpm() -> f32 {
///         90.
///     }
///
///     fn beats_per_measure() -> f32 {
///         5.
///     }
///
///     fn notes_per_measure() -> f32 {
///         8.
///     }
/// }
/// ```

trait SongParameters {
    /// Song title
    fn title() -> String;
    /// Returns self or a bundle with self
    fn new() -> impl Bundle;
    /// Song beats per minute
    fn bpm() -> f32;
    /// Number of beats per measures
    /// Used with bpm to compute length in seconds of a measure
    fn beats_per_measure() -> f32;
    /// Number of notes per measures
    /// Used with length in seconds of a measure to compute
    /// length of one note
    fn notes_per_measure() -> f32;
}

/// Used by `MusicSample` components to know when
/// they should spawn or not. It allows to build
/// song parts that spawn all the music samples linked to it.
#[derive(Component)]
struct SongPart {
    identifier: String,
}

/// The most basic component of a song
/// Holds an audio file and the audio channel
/// it will be queued in
#[derive(Component)]
struct MusicSample {
    file: Handle<AudioSource>,
    audio_channel: AudioChannels,
}

/// Lists all music audio channels
pub enum AudioChannels {
    Rhythm,
    Bass,
    Melody,
    Extra,
}

/// Audio channel for all rythmic samples
#[derive(Resource)]
struct RhythmAudioChannel;

/// Audio channel for all melodic samples
#[derive(Resource)]
struct MelodyAudioChannel;

/// Audio channel for all bass samples
#[derive(Resource)]
struct BassAudioChannel;

/// Audio channel for all misc samples
#[derive(Resource)]
struct ExtraAudioChannel;

/// A message when read, launches a
/// specific song and a specific music part
/// using identifiers of the said song/part
/// If identifiers does not link to a song, nothing happens.
#[derive(Message)]
pub struct PlaySong {
    pub song_title: String,
    pub part: String,
}

/// A message to stop all samples in all music audio channels
#[derive(Message)]
struct StopMusic;

/// A resource set to true if a PlaySong message is read
/// Allows to wait for music measure to finish before queuing new song/song part
/// Set to false after music samples are updated
#[derive(Resource)]
struct MusicUpdated(bool);

pub fn plugin(app: &mut App) {
    app.add_message::<PlaySong>();
    app.add_message::<StopMusic>();
    app.add_audio_channel::<RhythmAudioChannel>();
    app.add_audio_channel::<MelodyAudioChannel>();
    app.add_audio_channel::<BassAudioChannel>();
    app.add_audio_channel::<ExtraAudioChannel>();
    app.insert_resource(MusicUpdated(false));
    app.add_systems(
        Update,
        (
            despawn_old_song_part,
            despawn_old_music,
            set_music_updated,
            set_game_tempo,
            play_audios.run_if(time_to_update),
        )
            .chain(),
    );
    app.add_plugins(introduction::plugin);
}

/// Update game tempo with song tempo
fn set_game_tempo(song: Single<&Song, Added<Song>>, mut game_tempo: ResMut<GameTempo>) {
    let song = song.into_inner();

    println!("Tempo {:?}", song.bpm);

    game_tempo.bpm = song.bpm;
    game_tempo.notes_per_measure = song.notes_per_measure;
    game_tempo.beats_per_measure = song.beats_per_measure;
}

/// Despawn old song component when a new song is queued
fn despawn_old_music(
    mut commands: Commands,
    songs: Query<Entity, With<Song>>,
    mut events: MessageReader<PlaySong>,
) {
    for entity in songs {
        for _ in events.read() {
            commands.entity(entity).despawn();
        }
    }
}

/// Despawn old song part component when new song is queued
fn despawn_old_song_part(
    mut commands: Commands,
    song_parts: Query<Entity, With<SongPart>>,
    mut events: MessageReader<PlaySong>,
) {
    for entity in song_parts {
        for _ in events.read() {
            commands.entity(entity).despawn();
        }
    }
}

/// Set `MusicUpdate` resource to true if new song is queued
fn set_music_updated(mut events: MessageReader<PlaySong>, mut music_updated: ResMut<MusicUpdated>) {
    for _ in events.read() {
        music_updated.0 = true;
    }
}

/// Stop all samples and despawn `Song` component
/// if `StopMusic` message received
fn stop_music<T: Component + SongParameters>(
    mut commands: Commands,
    mut events: MessageReader<StopMusic>,
    songs: Query<Entity, With<T>>,
) {
    for entity in songs {
        for _ in events.read() {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawns `Song` based on song title
/// received in a `PlaySong` message
fn spawn_current_song<T: Component + SongParameters>(
    mut commands: Commands,
    mut events: MessageReader<PlaySong>,
    songs: Query<&Song, With<T>>,
) {
    for event in events.read() {
        if T::title() == event.song_title && songs.is_empty() {
            commands.spawn(T::new()).with_children(|parent| {
                parent.spawn(Song {
                    bpm: T::bpm(),
                    beats_per_measure: T::beats_per_measure(),
                    notes_per_measure: T::notes_per_measure(),
                });
            });
        }
    }
}

/// First stop audios and set music_updated to false
/// than queue all music samples in respective audio channels
fn play_audios(
    mut music_updated: ResMut<MusicUpdated>,
    music_samples: Query<&MusicSample>,
    rhythm_channel: Res<AudioChannel<RhythmAudioChannel>>,
    bass_channel: Res<AudioChannel<BassAudioChannel>>,
    melody_channel: Res<AudioChannel<MelodyAudioChannel>>,
    extra_channel: Res<AudioChannel<ExtraAudioChannel>>,
) {
    if !music_samples.is_empty() {
        music_updated.0 = false;
        rhythm_channel.stop();
        bass_channel.stop();
        melody_channel.stop();
        extra_channel.stop();
    }

    music_samples.par_iter().for_each(|music_sample| {
        match music_sample.audio_channel {
            AudioChannels::Rhythm => rhythm_channel.play(music_sample.file.to_owned()).looped(),
            AudioChannels::Bass => bass_channel.play(music_sample.file.to_owned()).looped(),
            AudioChannels::Melody => melody_channel.play(music_sample.file.to_owned()).looped(),
            AudioChannels::Extra => extra_channel.play(music_sample.file.to_owned()).looped(),
        };
    });
}

/// Returns true if music has been updated (`MusicUpdated`)
/// main_tick timer has ended
/// and music measure has ended
/// Used as a run condition for play_audios system
/// to avoid cutting music samples in the middle
fn time_to_update(
    music_updated: Res<MusicUpdated>,
    main_tick: Res<MainTick>,
    main_tick_counter: Res<MainTickCounter>,
    tick_delta: Res<TickDelta>,
) -> bool {
    music_updated.0
        && main_tick.timer.just_finished()
        && main_tick_counter.value as f32 % (tick_delta.measure / tick_delta.note) == 0.
}
