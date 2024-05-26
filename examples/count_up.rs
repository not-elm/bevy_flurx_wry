//! Minimum example showing how to use ipc communication.

use std::path::PathBuf;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_flurx::prelude::*;
use bevy_flurx_wry::prelude::*;


#[derive(Resource, Debug)]
struct Count(usize);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window{
                    resolution: WindowResolution::new(200., 200.),
                    ..default()
                }),
                ..default()
            }),
            FlurxWryPlugin {
                local_root: PathBuf::from("ui").join("count_up")
            }
        ))
        .insert_resource(Count(0))
        .add_systems(Startup, spawn_webview)
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>
){
    commands.entity(window.single()).insert(WryWebViewBundle{
        ipc_handlers: ipc_handlers![
            count_up
        ],
        ..default()
    });
}

#[command]
fn count_up() -> ActionSeed<(), usize>{
    once::run(|mut count: ResMut<Count>|{
        count.0 += 1;
        count.0
    })
}