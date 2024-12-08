//!  Testing to define `ipc-command` via macro.

use bevy_flurx::action::once;
use bevy_flurx::prelude::ActionSeed;
use bevy_flurx_ipc::component::IpcHandlers;
use bevy_flurx_ipc_macro::command;

#[command]
fn hello() -> ActionSeed{
    once::run(||{})
}

fn main(){
    IpcHandlers::new(hello);
    IpcHandlers::default().with(hello);
}