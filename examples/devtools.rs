use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, Entity, In, Query, With};
use bevy::utils::default;
use bevy::window::PrimaryWindow;

use bevy_flurx::action::{Action, once};
use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin
        ))
        .add_systems(Startup, spawn_webview)
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.entity(primary_window.single()).insert(WryWebViewBundle {
        // show assets/ui/devtools/index.html
        uri: Uri::LocalRoot("devtools".to_string()),
        use_devtools: UseDevtools(true),
        is_open_devtools: IsOpenDevtools(true),
        ipc_handlers: ipc_handlers![
            change_devtools_state
        ],
        ..default()
    });
}

#[command]
fn change_devtools_state(is_open: bool) -> Action<bool> {
    once::run(feedback_devtools).with(is_open)
}

fn feedback_devtools(
    In(is_open): In<bool>,
    mut view: Query<&mut IsOpenDevtools>,
) {
    view.single_mut().0 = is_open;
}
