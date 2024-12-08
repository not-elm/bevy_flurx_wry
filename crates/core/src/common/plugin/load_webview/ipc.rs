use bevy::ecs::system::SystemParam;
use bevy::log;
use bevy::prelude::{Entity, Res};
use serde::Deserialize;
use wry::WebViewBuilder;

use bevy_flurx_ipc::ipc_commands::{IpcCommand, IpcCommands, Payload};
use bevy_flurx_ipc::prelude::{IpcRawEvent, IpcRawEventBody, IpcRawEvents};

#[derive(SystemParam)]
pub(crate) struct IpcHandlerParams<'w> {
    ipc_commands: Res<'w, IpcCommands>,
    ipc_raw_events: Res<'w, IpcRawEvents>,
}

#[derive(Deserialize)]
#[serde(tag = "type", content = "message")]
enum IpcMessage {
    Command(Payload),
    Event(IpcRawEventBody),
}

impl IpcHandlerParams<'_> {
    pub(crate) fn feed_ipc<'a>(&self, webview_entity: Entity, builder: WebViewBuilder<'a>) -> WebViewBuilder<'a> {
        let ipc_commands = self.ipc_commands.clone();
        let ipc_raw_events = self.ipc_raw_events.clone();

        builder.with_ipc_handler(move |request| {
            match serde_json::from_str::<IpcMessage>(request.body()) {
                Ok(IpcMessage::Command(payload)) => {
                    ipc_commands.push(IpcCommand {
                        entity: webview_entity,
                        payload,
                    });
                }
                Ok(IpcMessage::Event(payload)) => {
                    ipc_raw_events.push(IpcRawEvent{
                        webview_entity,
                        body: payload
                    });
                }
                Err(e) => {
                    log::error!("failed deserialize ipc message: {e}");
                }
            }
        })
    }
}