//! Minimum example showing how to create a webview as child in the window.


use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin::default()
        ))
        .add_systems(Startup, (
            spawn_webview,
            spawn_camera
        ))
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.spawn((
        WryWebViewBundle{
            uri: WebviewUri::new("https://bevyengine.org/"),
            ..default()
        },
        AsChildBundle {
            parent: ParentWindow(window.single()),
            bounds: Bounds {
                size: Vec2::new(500., 500.),
                position: Vec2::new(100., 100.),
                min_size: Vec2::new(100., 100.),
            },
            resizable: Resizable(true),
        },
    ));
}

fn spawn_camera(
    mut commands: Commands
) {
    // The webview is rendered without a camera,
    // but the toolbar is rendered using bevy_ui, so a camera is required.
    commands.spawn(Camera2dBundle::default());
}