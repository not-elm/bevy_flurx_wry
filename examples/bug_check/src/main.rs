//! This is code for bug checking during development.

use std::fmt::Debug;
use std::path::PathBuf;

use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::Serialize;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use serde::Deserialize;

use bevy_flurx_wry::prelude::*;

#[derive(Component)]
struct WebviewWindow;

#[derive(Deserialize, Event, Debug)]
#[allow(dead_code)]
struct OnClickOnWebview {
    pub x: u32,
    pub y: u32
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            FlurxWryPlugin {
                local_root: PathBuf::from("ui").join("bug_check")
            },
            AppGetNameApiPlugin,
            AppGetVersionApiPlugin,
            AppExitApiPlugin,
            LogPrintlnApiPlugin,
        ))
        .add_ipc_event::<OnClickOnWebview>("onclick")
        .add_systems(Startup, (
            spawn_camera,
            spawn_webview
        ))
        .add_systems(Update, (
            event_console_output::<IpcEvent<OnClickOnWebview>>,
            event_console_output::<DownloadStarted>,
            event_console_output::<DownloadCompleted>,
            event_console_output::<NewWindowOpened>,
            event_console_output::<DragEntered>,
            event_console_output::<DragOver>,
            event_console_output::<Dropped>,
            event_console_output::<DragLeft>,
            event_console_output::<Navigated>,
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
            // uri: WebviewUri::relative_local("second.html"),
            // uri: WebviewUri::new("https://bevyengine.org/"),
            use_devtools: UseDevtools(true),
            is_open_devtools: IsOpenDevtools(true),
            ..default()
        },
        AsChildBundle {
            parent: ParentWindow(primary_window.single()),
            bounds: Bounds {
                size: Vec2::new(500., 500.),
                position: Vec2::new(100., 100.),
                ..default()
            },
            ..default()
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