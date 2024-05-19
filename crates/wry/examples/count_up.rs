use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, ClearColor, Color, Commands, Entity, Query, ResMut, Resource, With};
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use bevy_flurx::action::once;
use bevy_flurx::prelude::ActionSeed;

use bevy_flurx_wry::prelude::*;

#[derive(Resource)]
struct Count(usize);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin
        ))
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(Count(0))
        .add_systems(Startup, (
            spawn_camera,
            spawn_webview
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
    commands.entity(primary_window.single()).insert(WryWebViewBundle {
        // show assets/ui/count_up/index.html
        uri: Uri::LocalRoot("count_up".to_string()),
        ipc_handlers: ipc_handlers![
            count_up
        ],
        ..default()
    });
}

#[command]
fn count_up() -> ActionSeed<(), usize> {
    once::run(|mut count: ResMut<Count>| {
        count.0 += 1;
        count.0
    })
}

