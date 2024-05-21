//! Defines the plugin to apply ipc communication.


use bevy::app::{App, Update};
use bevy::prelude::{Commands, Entity, Event, EventWriter, In, Plugin, Query, Reflect, Res};
use bevy_flurx::FlurxPlugin;
use bevy_flurx::prelude::{Map, once, Pipe, Reactor};
use serde::{Deserialize, Serialize};

use crate::component::IpcHandlers;
use crate::ipc_command_queue::IpcCommands;


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


/// The core plugin for IPC communication between `Webview` and `bevy`.
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
    queue: Res<IpcCommands>,
    handlers: Query<&IpcHandlers>,
) {
    for ipc in queue.take_commands() {
        let Ok(invoke) = handlers.get(ipc.entity) else {
            continue;
        };
        if let Some(seed) = invoke.get_action_seed(&ipc.payload.id, ipc.payload.params) {
            let entity = ipc.entity;
            let resolve_id = ipc.payload.resolve_id;
            commands.spawn(Reactor::schedule(move |task| async move {
                task.will(Update, seed
                    .map(move |output| (entity, resolve_id, output))
                    .pipe(once::run(resolve)),
                ).await;
            }));
        }
    }
}

fn resolve(
    In((entity, resolve_id, output)): In<(Entity, usize, String)>,
    mut ew: EventWriter<IpcResolveEvent>,
) {
    ew.send(IpcResolveEvent {
        entity,
        resolve_id,
        output,
    });
}