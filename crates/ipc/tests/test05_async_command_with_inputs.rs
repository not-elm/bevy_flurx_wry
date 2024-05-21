use bevy::prelude::{In, Update};
use bevy_flurx::action::once;
use bevy_flurx::task::ReactiveTask;
use serde::Deserialize;

use bevy_flurx_ipc::ipc_handlers;
use bevy_flurx_ipc::prelude::WebviewEntity;
use bevy_flurx_ipc_macro::command;

#[command]
async fn hello(In(num): In<u32>) -> String {
    num.to_string()
}

#[derive(Deserialize)]
struct Test;

#[command]
async fn with_task1(In(message): In<String>, task: ReactiveTask) -> String {
    task.will(Update, once::run(move || message.to_string())).await
}

#[command]
async fn with_task2(In((_, message)): In<(Test, String)>, task: ReactiveTask) -> String {
    task.will(Update, once::run(move || message.to_string())).await
}

#[command]
async fn with_entity_and_task(In(message): In<String>, _: WebviewEntity, task: ReactiveTask) -> usize {
    task.will(Update, once::run(move || message.len())).await
}

#[command]
async fn with_expand_entity_and_task(In(_): In<String>, WebviewEntity(entity): WebviewEntity, task: ReactiveTask) -> u32 {
    task.will(Update, once::run(move || entity.index())).await
}

fn main() {
    ipc_handlers![
        hello,
        with_task1,
        with_task2,
        with_entity_and_task,
        with_expand_entity_and_task
    ];
}