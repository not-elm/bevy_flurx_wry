//! This Example show how to listen the [`IpcEvent`].

use std::path::PathBuf;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use serde::Deserialize;

use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin {
                local_root: PathBuf::from("ui").join("event_listen")
            }
        ))
        .add_ipc_event::<MessageFromWebview>("message")
        .add_systems(Startup, spawn_webview)
        .add_systems(Update, read_webview_message)
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.entity(window.single()).insert(WryWebViewBundle::default());
}

#[derive(Deserialize, Debug)]
struct MessageFromWebview {
    message: String,
}

fn read_webview_message(
    mut er: EventReader<IpcEvent<MessageFromWebview>>
) {
    for e in er.read() {
        println!("webview message: {}", e.payload.message);
    }
}