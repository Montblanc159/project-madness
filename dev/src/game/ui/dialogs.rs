use bevy::{color::palettes::css::BLACK, prelude::*};

#[derive(Component)]
struct DialogImage;

#[derive(Component)]
struct DialogSource;

#[derive(Component)]
struct DialogBody;

#[derive(Component)]
struct DialogContainer;

#[derive(Message)]
pub struct DialogEvent {
    pub source: String,
    // pub image: Handle<Image>,
    pub image: String,
    pub body: String,
}

#[derive(Resource)]
struct DialogTimer {
    value: Timer,
}

pub fn plugin(app: &mut App) {
    app.add_message::<DialogEvent>();
    app.insert_resource(DialogTimer {
        value: Timer::from_seconds(5., TimerMode::Once),
    });
    app.add_systems(Startup, spawn_dialog_box);
    app.add_systems(Update, show_dialog);
}

fn spawn_dialog_box(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut entity = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(0),
            display: Display::None,
            ..Default::default()
        },
        ImageNode {
            image: asset_server.load("textures/ui/dialog-box.png"),
            image_mode: NodeImageMode::Stretch,
            ..Default::default()
        },
        DialogContainer,
    ));

    entity.with_children(|child_commands| {
        child_commands.spawn((Text::new("Image"), TextColor(BLACK.into()), DialogImage));
        child_commands.spawn((Text::new("Name"), TextColor(BLACK.into()), DialogSource));
        child_commands.spawn((Text::new("Body"), TextColor(BLACK.into()), DialogBody));
    });
}

fn show_dialog(
    mut dialog_events: MessageReader<DialogEvent>,
    mut dialog_timer: ResMut<DialogTimer>,
    time: Res<Time>,
    dialog_container: Single<&mut Node, With<DialogContainer>>,
    // dialog_image: Single<&mut ImageNode, With<DialogImage>>,
    dialog_image: Single<
        &mut Text,
        (
            With<DialogImage>,
            Without<DialogSource>,
            Without<DialogBody>,
        ),
    >,
    dialog_source: Single<
        &mut Text,
        (
            With<DialogSource>,
            Without<DialogImage>,
            Without<DialogBody>,
        ),
    >,
    dialog_body: Single<
        &mut Text,
        (
            With<DialogBody>,
            Without<DialogImage>,
            Without<DialogSource>,
        ),
    >,
) {
    let mut container = dialog_container.into_inner();
    let mut image = dialog_image.into_inner();
    let mut source = dialog_source.into_inner();
    let mut body = dialog_body.into_inner();

    container.display = Display::None;

    if !dialog_timer.value.just_finished() {
        container.display = Display::Block;
        dialog_timer.value.tick(time.delta());
    }

    for event in dialog_events.read() {
        dialog_timer.value.reset();

        // image.image = event.image.clone();
        image.0 = event.image.clone();
        source.0 = event.source.clone();
        body.0 = event.body.clone();
    }
}
