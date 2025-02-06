//! This example demonstrates how to create a child window with a webview using [`bevy_child_window`].

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_child_window::{ChildWindowPlugin, ParentWindow};
use bevy_webview_wry::prelude::*;

fn main() {
    let mut app = App::new();
    app
        .add_plugins((
            DefaultPlugins,
            ChildWindowPlugin,
            WebviewWryPlugin::default(),
        ))
        .add_systems(Startup, spawn_child_window)
        .run();
}

fn spawn_child_window(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.spawn((
        Window {
            title: "Child Window".to_string(),
            resolution: WindowResolution::new(500., 500.),
            ..default()
        },
        WebviewUri::new("https://bevyengine.org/"),
        ParentWindow(window.single()),
    ));
}

