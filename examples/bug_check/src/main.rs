//! This is code for bug checking during development.
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy_flurx::prelude::*;
use bevy_flurx_wry::prelude::*;
use std::path::PathBuf;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(500., 500.),
                #[cfg(target_os = "macos")]
                composite_alpha_mode: bevy::window::CompositeAlphaMode::PostMultiplied,
                ..default()
            }),
            ..default()
        }),
        FlurxPlugin,
        FlurxWryPlugin {
            local_root: PathBuf::from("ui").join("bug_check"),
        },
        // LogPrintlnApiPlugin,
        // AllWebWindowPlugins,
        // AllHttpPlugins,
    ));

    #[cfg(target_os = "macos")]
    app.insert_resource(ClearColor(Color::NONE));

    app
        .add_systems(Startup, spawn_webview)
        .run();
}

fn spawn_webview(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>) {
    commands
        .spawn((
            WebviewUri::default(),
            UseDevtools(true),
            ParentWindow(window.single()),
            Bounds {
                size: Vec2::splat(250.),
                ..default()
            },
        ));
}

