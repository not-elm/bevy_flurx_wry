//! This example show how to emit the event to a webview side.

use std::path::PathBuf;
use std::time::Duration;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_flurx_wry::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin {
                local_root: PathBuf::from("ui").join("event_emit")
            }
        ))
        .insert_resource(CountTimer(Timer::new(Duration::from_secs(1), TimerMode::Repeating)))
        .add_systems(Startup, spawn_webview)
        .add_systems(Update, emit_event)
        .run();
}

#[derive(Resource)]
struct CountTimer(Timer);

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.entity(window.single()).insert(WryWebViewBundle::default());
}

fn emit_event(
    mut timer: ResMut<CountTimer>,
    mut views: Query<&mut EventEmitter>,
    mut count: Local<usize>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).finished() {
        *count += 1;
        for mut emitter in views.iter_mut() {
            emitter.emit("count_event", &serde_json::json!({
                "count" : *count
            }));
        }
    }
}