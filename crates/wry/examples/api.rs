use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, Component, default, KeyCode, Query, Res, Update, Window, With};
use bevy::window::WindowResolution;

use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin
        ))
        .add_systems(Startup, spawn_webview)
        .add_systems(Update, show_webview_window)
        .run();
}

#[derive(Component)]
struct WebviewWindow;

fn spawn_webview(
    mut commands: Commands
) {
    commands.spawn((
        WebviewWindow,
        Window{
            resolution: WindowResolution::new(200., 200.),
            ..default()
        },
        WryWebViewBundle {
            uri: Uri::LocalRoot("api".to_string()),
            ..default()
        }
    ));
}

fn show_webview_window(
    mut window: Query<&mut Window, With<WebviewWindow>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyQ) {
        window.single_mut().visible = true;
    }
}