use std::time::Duration;

use bevy::app::{App, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, Entity, Local, Query, Res, ResMut, Resource, Update, With};
use bevy::time::{Time, Timer, TimerMode};
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use serde_json::json;

use bevy_flurx_wry::prelude::*;

#[derive(Resource)]
struct CountTimer(Timer);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin
        ))
        .insert_resource(CountTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)))
        .add_systems(Startup, spawn_webview)
        .add_systems(Update, emit)
        .run();
}

fn spawn_webview(
    mut commands: Commands,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.entity(primary_window.single()).insert(
        WryWebViewBundle {
            // show assets/ui/event_emit/index.html
            uri: Uri::LocalRoot("event_emit".to_string()),
            ..default()
        }
    );
}

fn emit(
    mut timer: ResMut<CountTimer>,
    mut emitter: Query<&mut EventEmitter>,
    mut count: Local<usize>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).finished() {
        *count += 1;
        emitter.single_mut().emit("count", json!({
            "count": *count
        }));
    }
}