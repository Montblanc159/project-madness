use bevy::prelude::*;

const TITLE: &str = "Introduction";
const BPM: f32 = 120.;
const BEATS_PER_MEASURE: f32 = 4.;
const NOTES_PER_MEASURE: f32 = 8.;

#[derive(Component)]
struct IntroductionSong;

impl super::SongParameters for IntroductionSong {
    fn title() -> String {
        TITLE.into()
    }

    fn bpm() -> f32 {
        BPM
    }

    fn beats_per_measure() -> f32 {
        BEATS_PER_MEASURE
    }

    fn notes_per_measure() -> f32 {
        NOTES_PER_MEASURE
    }

    fn new() -> impl Bundle {
        Self
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            super::stop_music::<IntroductionSong>,
            super::spawn_current_song::<IntroductionSong>,
            spawn_intro_part,
            spawn_main_part,
            (
                spawn_main_melody_sample,
                spawn_main_rhythm_sample,
                spawn_main_bass_sample,
                spawn_main_extra_sample,
            ),
        )
            .chain()
            .after(super::despawn_old_song_part),
    );
}

fn spawn_intro_part(
    mut commands: Commands,
    mut events: MessageReader<super::PlaySong>,
    song: Single<Entity, Added<IntroductionSong>>,
) {
    let song = song.into_inner();

    for event in events.read() {
        if event.part == String::from("intro") {
            commands.entity(song).with_children(|parent| {
                parent.spawn(super::SongPart {
                    identifier: "intro".into(),
                });
            });
        }
    }
}

fn spawn_main_part(
    mut commands: Commands,
    mut events: MessageReader<super::PlaySong>,
    song: Single<Entity, Added<IntroductionSong>>,
) {
    let song = song.into_inner();

    for event in events.read() {
        if event.part == String::from("main") {
            commands.entity(song).with_children(|parent| {
                parent.spawn(super::SongPart {
                    identifier: "main".into(),
                });
            });
        }
    }
}

fn spawn_main_melody_sample(
    mut commands: Commands,
    parts: Query<(Entity, &super::SongPart), Added<super::SongPart>>,
    song: Single<&Children, With<IntroductionSong>>,
    asset_server: Res<AssetServer>,
) {
    let song = song.into_inner();

    for child in song {
        if let Ok(part) = parts.get(*child) {
            let (entity, song_part) = part;

            if [String::from("intro"), String::from("main")].contains(&song_part.identifier) {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn(super::MusicSample {
                        file: asset_server.load("audios/music/introduction/main-melody.ogg"),
                        audio_channel: super::AudioChannels::Melody,
                    });
                });
            }
        }
    }
}

fn spawn_main_rhythm_sample(
    mut commands: Commands,
    parts: Query<(Entity, &super::SongPart), Added<super::SongPart>>,
    song: Single<&Children, With<IntroductionSong>>,
    asset_server: Res<AssetServer>,
) {
    let song = song.into_inner();

    for child in song {
        if let Ok(part) = parts.get(*child) {
            let (entity, song_part) = part;

            if [String::from("main")].contains(&song_part.identifier) {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn(super::MusicSample {
                        file: asset_server.load("audios/music/introduction/main-rhythm.ogg"),
                        audio_channel: super::AudioChannels::Rhythm,
                    });
                });
            }
        }
    }
}

fn spawn_main_bass_sample(
    mut commands: Commands,
    parts: Query<(Entity, &super::SongPart), Added<super::SongPart>>,
    song: Single<&Children, With<IntroductionSong>>,
    asset_server: Res<AssetServer>,
) {
    let song = song.into_inner();

    for child in song {
        if let Ok(part) = parts.get(*child) {
            let (entity, song_part) = part;

            if [String::from("main")].contains(&song_part.identifier) {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn(super::MusicSample {
                        file: asset_server.load("audios/music/introduction/main-bass.ogg"),
                        audio_channel: super::AudioChannels::Bass,
                    });
                });
            }
        }
    }
}

fn spawn_main_extra_sample(
    mut commands: Commands,
    parts: Query<(Entity, &super::SongPart), Added<super::SongPart>>,
    song: Single<&Children, With<IntroductionSong>>,
    asset_server: Res<AssetServer>,
) {
    let song = song.into_inner();

    for child in song {
        if let Ok(part) = parts.get(*child) {
            let (entity, song_part) = part;

            if [String::from("main")].contains(&song_part.identifier) {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn(super::MusicSample {
                        file: asset_server.load("audios/music/introduction/main-extra.ogg"),
                        audio_channel: super::AudioChannels::Extra,
                    });
                });
            }
        }
    }
}
