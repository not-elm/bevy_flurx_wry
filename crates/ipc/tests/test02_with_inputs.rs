//!  Testing to define `ipc-command` from a function with input args via macro.

use bevy_ecs::prelude::In;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;
use bevy_flurx_ipc::component::IpcHandlers;
use bevy_flurx_ipc::prelude::WebviewEntity;
use bevy_flurx_ipc_macro::command;

#[command]
fn hello1(name: In<String>) -> Action<String> {
    once::run(|In(_): In<String>| {}).with(name.0)
}

#[command]
fn hello2(In(name): In<String>) -> Action<String> {
    once::run(|In(_): In<String>| {}).with(name)
}

#[command]
fn hello3(name: In<String>, WebviewEntity(_): WebviewEntity) -> Action<String> {
    once::run(|In(_): In<String>| {}).with(name.0)
}

fn main() {
    IpcHandlers::new(hello1)
        .with(hello2)
        .with(hello3);
}