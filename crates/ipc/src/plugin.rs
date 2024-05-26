//! Defines the plugin to apply ipc communication.



use bevy::app::{App, Update};
use bevy::prelude::{Commands, Entity, Event, Plugin, Query, Reflect, Res};
use bevy_flurx::FlurxPlugin;
use bevy_flurx::prelude::Reactor;
use serde::{Deserialize, Serialize};

use crate::component::IpcHandlers;
use crate::ipc_commands::IpcCommands;

/// The event signals the end of ipc processing.
#[derive(Event, Eq, PartialEq, Clone, Serialize, Deserialize, Reflect)]
pub struct IpcResolveEvent {
    /// The entity attached to [`IpcHandlers`](bevy::prelude::IpcHandlers) that execute ipc.
    pub entity: Entity,

    /// The id used to resolve asynchronous ipc.
    pub resolve_id: usize,

    /// The serialized output value.
    pub output: String,
}


/// The common plugin for IPC communication between `Webview` and `bevy`.
pub struct FlurxIpcPlugin;

impl Plugin for FlurxIpcPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxPlugin>() {
            app.add_plugins(FlurxPlugin);
        }

        app
            .add_event::<IpcResolveEvent>()
            .init_resource::<IpcCommands>()
            .add_systems(Update, receive_ipc_commands);
    }
}

fn receive_ipc_commands(
    mut commands: Commands,
    ipc_commands: Res<IpcCommands>,
    handlers: Query<&IpcHandlers>,
) {
    for cmd in ipc_commands.take_commands() {
        let Ok(handlers) = handlers.get(cmd.entity) else {
            continue;
        };
        if let Some(ipc_fn) = handlers.get(&cmd.payload.id) {
            commands.spawn(Reactor::schedule(move |task| async move {
                ipc_fn(task, cmd).await;
            }));
        }
    }
}

