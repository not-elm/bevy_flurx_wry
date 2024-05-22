use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, default, Entity, KeyCode, Query, Res, Update, Window, With};
use bevy::window::PrimaryWindow;

use bevy_flurx_wry::as_child::{AsChild, Bounds, ParentWindow};
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
    mut commands: Commands,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.spawn((
        WebviewWindow,
        WryWebViewBundle {
            uri: Uri::LocalRoot("api".to_string()),
            ..default()
        },
        AsChild {
            parent: ParentWindow(primary_window.single()),
            bounds: Bounds {
                position: Vec2::new(100., 100.),
                size: Vec2::new(500., 500.),
            },
        },
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