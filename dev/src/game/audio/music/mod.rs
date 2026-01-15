use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioSource};

use crate::game::tick::{GameTempo, MainTick, MainTickCounter, TickDelta};

mod introduction;

#[derive(Component)]
struct Song {
    bpm: f32,
    beats_per_measure: f32,
    notes_per_measure: f32,
}

trait SongParameters {
    fn title() -> String;
    fn new() -> impl Bundle;
    fn bpm() -> f32;
    fn beats_per_measure() -> f32;
    fn notes_per_measure() -> f32;
}

#[derive(Component)]
struct SongPart {
    identifier: String,
}

#[derive(Component)]
struct MusicSample {
    file: Handle<AudioSource>,
    audio_channel: AudioChannels,
}

pub enum AudioChannels {
    Rhythm,
    Bass,
    Melody,
    Extra,
}

#[derive(Resource)]
struct RhythmAudioChannel;

#[derive(Resource)]
struct MelodyAudioChannel;

#[derive(Resource)]
struct BassAudioChannel;

#[derive(Resource)]
struct ExtraAudioChannel;

#[derive(Message)]
pub struct PlaySong {
    pub song_title: String,
    pub part: String,
}

#[derive(Message)]
struct StopMusic;

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

fn set_game_tempo(song: Single<&Song, Added<Song>>, mut game_tempo: ResMut<GameTempo>) {
    let song = song.into_inner();

    println!("Tempo {:?}", song.bpm);

    game_tempo.bpm = song.bpm;
    game_tempo.notes_per_measure = song.notes_per_measure;
    game_tempo.beats_per_measure = song.beats_per_measure;
}

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

fn set_music_updated(mut events: MessageReader<PlaySong>, mut music_updated: ResMut<MusicUpdated>) {
    for _ in events.read() {
        music_updated.0 = true;
    }
}

fn stop_music<T: Component + SongParameters>(
    mut commands: Commands,
    mut events: MessageReader<StopMusic>,
    songs: Query<Entity, With<T>>,
    rhythm_channel: Res<AudioChannel<RhythmAudioChannel>>,
    bass_channel: Res<AudioChannel<BassAudioChannel>>,
    melody_channel: Res<AudioChannel<MelodyAudioChannel>>,
    extra_channel: Res<AudioChannel<ExtraAudioChannel>>,
) {
    for entity in songs {
        for _ in events.read() {
            commands.entity(entity).despawn();
            rhythm_channel.stop();
            bass_channel.stop();
            melody_channel.stop();
            extra_channel.stop();
        }
    }
}

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
