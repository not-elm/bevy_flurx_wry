//! Testing to pass async ipc-commands to the `ipc_handlers!`.

use bevy_app::prelude::Update;
use bevy_ecs::prelude::In;
use bevy_flurx::action::{delay, once};
use bevy_flurx::task::ReactorTask;
use bevy_flurx_ipc::ipc_handlers;
use bevy_flurx_ipc::prelude::WebviewEntity;
use bevy_flurx_ipc_macro::command;

#[command(internal)]
async fn pattern1() -> String {
    "hello".to_string()
}

#[command(internal)]
async fn pattern2(In(_): In<()>) {}

#[command(internal)]
async fn pattern3(_webview_entity: WebviewEntity) {}

#[command(internal)]
async fn pattern4(task: ReactorTask) -> String {
    task.will(Update, once::run(|| "hello".to_string())).await
}

#[command(internal)]
async fn pattern5(
    In(_args): In<()>,
    WebviewEntity(_entity): WebviewEntity,
) {}

#[command(internal)]
async fn pattern6(In(frames): In<usize>, task: ReactorTask) {
    task.will(Update, delay::frames().with(frames)).await;
}

#[command(internal)]
async fn pattern7(
    In(_args): In<()>,
    _task: ReactorTask,
) {}

#[command(internal)]
async fn pattern8(
    In(_args): In<()>,
    _entity: WebviewEntity,
    _task: ReactorTask,
) {}

fn main() {
    ipc_handlers![
        pattern1,
        pattern2,
        pattern3,
        pattern4,
        pattern5,
        pattern6,
        pattern7,
        pattern8,
    ];
}