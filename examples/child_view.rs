//! Minimum example showing how to create a webview as child in the window.

use std::path::PathBuf;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin {
                local_root: PathBuf::from("ui").join("child_view")
            }
        ))
        .add_systems(Startup, spawn_webview)
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.spawn((
        WebviewUri::default(),
        AsChildBundle {
            // Here, create a webview as child inside a given window.
            parent: ParentWindow(window.single()),
            bounds: Bounds {
                position: Vec2::new(100., 100.),
                size: Vec2::new(500., 500.),
                min_size: Vec2::new(100., 100.),
            },
            ..default()
        },
    ));

    commands.spawn((
        WebviewUri::new("https://bevyengine.org/"),
        AsChildBundle {
            parent: ParentWindow(window.single()),
            bounds: Bounds {
                position: Vec2::new(700., 100.),
                size: Vec2::new(500., 500.),
                min_size: Vec2::new(100., 100.),
            },
            ..default()
        },
    ));
}

