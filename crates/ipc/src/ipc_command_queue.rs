//! Defines the ipc commands and the queue to execute them.

use std::sync::{Arc, Mutex};
use bevy::prelude::{Entity, Resource};
use serde::Deserialize;


/// The ipc command queue that exists only one in the [`World`](bevy::prelude::World).
#[derive(Resource, Clone, Default)]
pub struct IpcCommandQueue(Arc<Mutex<Vec<IpcCommand>>>);

impl IpcCommandQueue {
    /// Push the [`IpcCommand`] into queue.
    ///
    /// The pushed command is automatically executed and output as [`IpcResolveEvent`](crate::prelude::IpcResolveEvent).
    ///
    #[inline(always)]
    pub fn push(&self, command: IpcCommand) {
        self.0.lock().unwrap().push(command);
    }
    
    #[inline(always)]
    pub(crate) fn take_commands(&self) -> Vec<IpcCommand>{
        std::mem::take(&mut self.0.lock().unwrap())
    }
}


/// The ipc command to execute.
///
/// [`IpcHandler`](crate::prelude::IpcHandler) must be spawned in the world to run this command.
#[derive(Deserialize)]
pub struct IpcCommand {
    /// The entity is attached to [`IpcHandlers`](crate::prelude::IpcHandlers) that executes this command.
    pub entity: Entity,

    /// The command info passed from `javascript`.
    pub body: IpcCommandBody,
}


/// The command info passed from `javascript`.
#[derive(Deserialize)]
pub struct IpcCommandBody {
    /// Ipc id
    pub id: String,

    /// The array of args passed from javascript.
    pub params: Vec<String>,

    /// This value is used when waiting for IPC asynchronously.
    ///
    /// You need to create `resolve_id` in  `javascript` and then need to can call `Promise::resolve` with id.
    pub resolve_id: usize,
}
