//! Testing to return result type from the actions.

use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::prelude::IpcHandlers;
use bevy_flurx_ipc_macro::command;

#[command(internal)]
fn action_command() -> Action<(), Result<String, String>> {
    once::run(|| Ok("hello".to_string())).with(())
}

#[command(internal)]
async fn async_command() -> Result<String, String> {
    Ok("hello".to_string())
}

fn main() {
    IpcHandlers::new([
        action_command,
        async_command,
    ]);
}