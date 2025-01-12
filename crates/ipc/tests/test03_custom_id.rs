//! Testing to edit the id of  ipc-command.

use bevy_flurx_ipc::prelude::IpcHandlers;
use bevy_flurx_ipc_macro::command;

#[command(id = "FLURX|TEST", internal)]
async fn hello() -> String {
    "hello".to_string()
}

fn main() {
    IpcHandlers::new([
        hello,
    ]);
    assert_eq!(hello().id(), "FLURX|TEST");
}