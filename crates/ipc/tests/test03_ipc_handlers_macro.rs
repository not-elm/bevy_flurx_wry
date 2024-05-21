use bevy::prelude::In;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;
use bevy_flurx_ipc::ipc_handlers;

use bevy_flurx_ipc_macro::command;

#[command]
fn hello(name: In<String>) -> Action<String> {
    once::run(|In(_): In<String>| {}).with(name.0)
}

#[command]
fn hello2(In((name, _num)): In<(String, usize)>) -> Action<String> {
    once::run(|In(_): In<String>| {}).with(name)
}

fn main() {
    ipc_handlers![];
    ipc_handlers![
        hello,
        hello2
    ];
}