use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, Commands, Text2dBundle, Window};
use bevy::text::{Text, TextStyle};
use bevy::utils::default;
use bevy::window::WindowResolution;

use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin
        ))
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
    commands.spawn(Text2dBundle {
        text: Text::from_section("Primary Window", TextStyle::default()),
        ..default()
    });
}

fn spawn_webview(
    mut commands: Commands,
) {
    commands.spawn((
        Window{
            resolution: WindowResolution::new(200., 200.),
            ..default()
        },
        WryWebViewBundle {
            // show assets/ui/sub_window/index.html
            uri: Uri::LocalRoot("sub_window".to_string()),
            ..default()
        }
    ));
}

