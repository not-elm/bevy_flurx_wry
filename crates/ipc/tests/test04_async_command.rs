use bevy::prelude::Update;
use bevy_flurx::action::once;
use bevy_flurx::task::ReactiveTask;

use bevy_flurx_ipc::ipc_handlers;
use bevy_flurx_ipc::prelude::WebviewEntity;
use bevy_flurx_ipc_macro::command;

#[command]
async fn hello() -> String {
    "hello".to_string()
}

#[command]
async fn with_task(task: ReactiveTask) -> String {
    task.will(Update, once::run(|| "hello".to_string())).await
}

#[command]
async fn with_entity_and_task(entity: WebviewEntity, task: ReactiveTask) -> u32 {
    task.will(Update, once::run(move || entity.0.index())).await
}

#[command]
async fn with_expand_entity_and_task(WebviewEntity(entity): WebviewEntity, task: ReactiveTask) -> u32 {
    task.will(Update, once::run(move || entity.index())).await
}

fn main() {
    ipc_handlers![
        hello,
        with_task,
        with_entity_and_task,
        with_expand_entity_and_task
    ];
}