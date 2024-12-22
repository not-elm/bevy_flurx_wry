//! Minimum example showing how to create a webview in the window.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin::default()
        ))
        .add_systems(Startup, spawn_webview)
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    // Converts the `Window` attached the entity into a webview window. 
    commands
        .entity(window.single())
        .insert(WebviewUri::new("https://bevyengine.org/"));
}

