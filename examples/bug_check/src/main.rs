//! This is code for bug checking during development.
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_flurx::prelude::*;
use bevy_flurx_wry::api::web_window::AllWebWindowPlugins;
use bevy_flurx_wry::prelude::*;
use std::path::PathBuf;

#[derive(Resource, Debug)]
struct Count(usize);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(200., 200.),
                    ..default()
                }),
                ..default()
            }),
            FlurxPlugin,
            FlurxWryPlugin {
                local_root: PathBuf::from("ui").join("bug_check")
            },
            AllWebWindowPlugins,
        ))
        .insert_resource(Count(0))
        .add_systems(Startup, spawn_webview)
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.entity(window.single()).insert((
        WebviewUri::default(),
        UseDevtools(true),
        ipc_handlers![
            count_up
        ],
    ));
}

#[command]
async fn count_up(task: ReactorTask) {
    task.will(Update, once::run(|mut count: ResMut<Count>| {
        count.0 += 1;
        println!("{}", count.0);
        count.0
    })).await;
}