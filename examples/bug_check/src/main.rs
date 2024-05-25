use std::fmt::Debug;
use std::path::PathBuf;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::Serialize;
use bevy::window::PrimaryWindow;

use bevy_flurx_wry::prelude::*;

#[derive(Component)]
struct WebviewWindow;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin {
                content_root: PathBuf::from("ui").join("bug_check")
            }
        ))
        .add_systems(Startup, (
            spawn_camera,
            spawn_webview
        ))
        .add_systems(Update, (
            event_console_output::<DownloadStarted>,
            event_console_output::<DownloadCompleted>,
            event_console_output::<NewWindowOpened>,
            event_console_output::<DragEntered>,
            event_console_output::<DragOver>,
            event_console_output::<Dropped>,
            event_console_output::<DragLeft>,
            event_console_output::<NavigationStarted>,
            test_event_emit.run_if(input_just_pressed(KeyCode::KeyR))
        ))
        .run();
}

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_webview(
    mut commands: Commands,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.spawn((
        WebviewWindow,
        WryWebViewBundle {
            uri: Uri::Remote("https://bevyengine.org/".to_string()),
            use_devtools: UseDevtools(true),
            is_open_devtools: IsOpenDevtools(true),
            ..default()
        },
        AsChild {
            parent: ParentWindow(primary_window.single()),
            bounds: Bounds {
                size: Vec2::new(500., 500.),
                // position: Vec2::new(100., 100.),
                ..default()
            },
            resizable: Resizable(true),
        },
        Toolbar {
            height: 20.,
            color: Color::rgb_u8(0x0E, 0x0E, 0x0E),
        }
    ));
}

fn test_event_emit(
    mut views: Query<&mut EventEmitter, With<WebviewWindow>>
) {
    #[derive(Serialize)]
    struct Payload {
        message: String,
    }

    views.single_mut().emit("test_event", Payload {
        message: "test message!".to_string()
    });
}


fn event_console_output<E: Event + Debug>(
    mut er: EventReader<E>
) {
    for e in er.read() {
        println!("{e:?}");
    }
}