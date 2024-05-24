use bevy_flurx_ipc::ipc_handlers;
use bevy_flurx_ipc_macro::command;

#[command(id="FLURX|TEST")]
async fn hello() -> String {
    "hello".to_string()
}

fn main() {
    ipc_handlers![
        hello,
    ];
    assert_eq!(hello().id(), "FLURX|TEST");
}