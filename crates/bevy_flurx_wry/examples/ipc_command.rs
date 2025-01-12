//!  This example show how to use `action-command` and `task-command`.
//!
//! This commands allows you to receive input from the webview and return output there

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_flurx::prelude::*;
use bevy_flurx_ipc::prelude::*;
use bevy_flurx_wry::prelude::*;
use std::path::PathBuf;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxWryPlugin {
                local_root: PathBuf::from("ui").join("ipc_command")
            }
        ))
        .add_systems(Startup, spawn_webview)
        .run();
}

#[derive(Component)]
struct Num(usize);

fn spawn_webview(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
) {
    commands.entity(window.single()).insert((
        Num(1),
        WebviewUri::default(),
        IpcHandlers::new([
            action_command,
            task_command
        ]),
    ));
}

/// Input args are optional.
///
/// For example, you could also define command as follows:
///
/// - `fn ipc_command()`
/// - `fn ipc_command(In(t): In<T>)`
/// - `fn ipc_command(WebviewEntity(entity): WebviewEntity)`
#[command]
fn action_command(
    In(n): In<usize>, // The input from javascript
    WebviewEntity(entity): WebviewEntity,
) -> Action<(Entity, usize), usize> {
    once::run(fibonacci).with((entity, n))
}

/// As with `action-command`, input args are optional.
#[command]
async fn task_command(
    In(n): In<usize>, // The input from javascript
    WebviewEntity(entity): WebviewEntity,
    task: ReactorTask,
) -> usize {
    task.will(Update, once::run(fibonacci).with((entity, n))).await
}

fn fibonacci(
    In((webview_entity, n)): In<(Entity, usize)>,
    mut views: Query<&mut Num>,
) -> usize {
    if let Ok(mut num) = views.get_mut(webview_entity) {
        let out = num.0 + n;
        num.0 = n;
        out
    } else {
        0
    }
}