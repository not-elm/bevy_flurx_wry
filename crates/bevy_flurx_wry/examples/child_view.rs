//! Minimum example showing how to create a webview as child in the window.

use bevy::prelude::*;
use bevy::window::{CompositeAlphaMode, PrimaryWindow};
use bevy_flurx_wry::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut app = App::new();

    #[cfg(target_os = "macos")]
    app.insert_resource(ClearColor(Color::NONE));

    app
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    #[cfg(target_os = "macos")]
                    composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                    ..default()
                }),
                ..default()
            }),
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
        ParentWindow(window.single()),
        Bounds {
            position: Vec2::new(100., 100.),
            size: Vec2::new(500., 500.),
            min_size: Vec2::new(100., 100.),
        },
    ));

    commands.spawn((
        WebviewUri::new("https://bevyengine.org/"),
        ParentWindow(window.single()),
        Bounds {
            position: Vec2::new(700., 100.),
            size: Vec2::new(500., 500.),
            min_size: Vec2::new(100., 100.),
        },
    ));
}

