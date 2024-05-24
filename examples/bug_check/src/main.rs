use std::fmt::Debug;
use std::path::PathBuf;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin {
                content_root: PathBuf::from("ui").join("bug_check")
            }
        ))
        .add_systems(Startup, spawn_webview)
        .add_systems(Update, (
            event_console_output::<DownloadStarted>,
            event_console_output::<DownloadCompleted>,
            event_console_output::<NewWindowOpened>,
            event_console_output::<DragEntered>,
            event_console_output::<DragOver>,
            event_console_output::<Dropped>,
            event_console_output::<DragLeft>,
            event_console_output::<NavigationStarted>,
        ))
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.entity(primary_window.single()).insert((
        WryWebViewBundle {
            // uri: Uri::Remote("https://bevyengine.org/".to_string()),
            use_devtools: UseDevtools(true),
            is_open_devtools: IsOpenDevtools(true),
            ..default()
        },
        // AsChild {
        //     parent: ParentWindow(primary_window.single()),
        //     bounds: Bounds {
        //         size: Vec2::new(500., 500.),
        //         ..default()
        //     },
        //     resizable: Resizable(true),
        // }
    ));
}

fn event_console_output<E: Event + Debug>(
    mut er: EventReader<E>
) {
    for e in er.read() {
        println!("{e:?}");
    }
}