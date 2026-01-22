use bevy::prelude::*;
use pipelines_ready::*;

#[derive(States, Default, Debug, Eq, PartialEq, Clone, Hash)]
pub enum LoadingState {
    InProgress,
    #[default]
    Done,
}

pub fn plugin(app: &mut App) {
    app.init_state::<LoadingState>();
    app.insert_resource(LoadingData::new(5));
    app.add_plugins(PipelinesReadyPlugin);
    app.add_systems(Startup, load_loading_screen);
    app.add_systems(
        Update,
        (
            set_loading_state,
            update_loading_data,
            display_loading_screen,
        ),
    );
}

// A resource that holds the current loading data.
#[derive(Resource, Debug, Default)]
pub struct LoadingData {
    // This will hold the currently unloaded/loading assets.
    pub loading_assets: Vec<UntypedHandle>,
    // Number of frames that everything needs to be ready for.
    // This is to prevent going into the fully loaded state in instances
    // where there might be a some frames between certain loading/pipelines action.
    confirmation_frames_target: usize,
    // Current number of confirmation frames.
    confirmation_frames_count: usize,
}

impl LoadingData {
    fn new(confirmation_frames_target: usize) -> Self {
        Self {
            loading_assets: Vec::new(),
            confirmation_frames_target,
            confirmation_frames_count: 0,
        }
    }
}

fn set_loading_state(
    mut loading_state: ResMut<NextState<LoadingState>>,
    loading_data: Res<LoadingData>,
    current_state: Res<State<LoadingState>>,
) {
    if !loading_data.loading_assets.is_empty() && *current_state.get() != LoadingState::InProgress {
        loading_state.set(LoadingState::InProgress);
    }
}

// Monitors current loading status of assets.
fn update_loading_data(
    mut loading_data: ResMut<LoadingData>,
    mut loading_state: ResMut<NextState<LoadingState>>,
    asset_server: Res<AssetServer>,
    pipelines_ready: Res<PipelinesReady>,
) {
    if !loading_data.loading_assets.is_empty() || !pipelines_ready.0 {
        // If we are still loading assets / pipelines are not fully compiled,
        // we reset the confirmation frame count.
        loading_data.confirmation_frames_count = 0;

        loading_data.loading_assets.retain(|asset| {
            asset_server
                .get_recursive_dependency_load_state(asset)
                .is_none_or(|state| !state.is_loaded())
        });

        // If there are no more assets being monitored, and pipelines
        // are compiled, then start counting confirmation frames.
        // Once enough confirmations have passed, everything will be
        // considered to be fully loaded.
    } else {
        loading_data.confirmation_frames_count += 1;
        if loading_data.confirmation_frames_count == loading_data.confirmation_frames_target {
            loading_state.set(LoadingState::Done);
        }
    }
}

// Marker tag for loading screen components.
#[derive(Component)]
struct LoadingScreen;

// Spawns the necessary components for the loading screen.
fn load_loading_screen(mut commands: Commands) {
    let text_style = TextFont {
        font_size: 67.0,
        ..default()
    };

    // Spawn the UI that will make up the loading screen.
    commands
        .spawn((
            Node {
                height: percent(100),
                width: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            LoadingScreen,
        ))
        .with_child((Text::new("Loading..."), text_style.clone()));
}

// Determines when to show the loading screen
fn display_loading_screen(
    mut loading_screen: Single<&mut Visibility, (With<LoadingScreen>, With<Node>)>,
    loading_state: Res<State<LoadingState>>,
) {
    let visibility = match loading_state.get() {
        LoadingState::InProgress => Visibility::Visible,
        LoadingState::Done => Visibility::Hidden,
    };

    **loading_screen = visibility;
}

mod pipelines_ready {
    use bevy::{
        prelude::*,
        render::{render_resource::*, *},
    };

    pub struct PipelinesReadyPlugin;
    impl Plugin for PipelinesReadyPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(PipelinesReady::default());

            // In order to gain access to the pipelines status, we have to
            // go into the `RenderApp`, grab the resource from the main App
            // and then update the pipelines status from there.
            // Writing between these Apps can only be done through the
            // `ExtractSchedule`.
            app.sub_app_mut(RenderApp)
                .add_systems(ExtractSchedule, update_pipelines_ready);
        }
    }

    #[derive(Resource, Debug, Default)]
    pub struct PipelinesReady(pub bool);

    fn update_pipelines_ready(mut main_world: ResMut<MainWorld>, pipelines: Res<PipelineCache>) {
        if let Some(mut pipelines_ready) = main_world.get_resource_mut::<PipelinesReady>() {
            pipelines_ready.0 = pipelines.waiting_pipelines().count() == 0;
        }
    }
}
