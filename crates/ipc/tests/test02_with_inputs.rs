use bevy::prelude::In;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;

use bevy_flurx_ipc::component::IpcHandlers;
use bevy_flurx_ipc_macro::command;

#[command]
fn hello(name: String) -> Action<String> {
    once::run(|In(_): In<String>| {}).with(name)
}

fn main() {
    IpcHandlers::new(hello);
}