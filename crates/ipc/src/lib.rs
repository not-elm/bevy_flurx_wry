//! This library facilitates communication between the `webview` and `bevy`.

use crate::ipc_commands::FlurxIpcCommandPlugin;
use crate::prelude::FlurxIpcEventPlugin;
use bevy::prelude::{App, Plugin};
use bevy_flurx::FlurxPlugin;
pub use bevy_flurx_ipc_macro::command;

pub mod component;
pub mod ipc_commands;
pub mod ipc_events;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{
        component::*,
        ipc_commands::*,
        ipc_events::*,
        FlurxIpcPlugin,
    };
    pub use bevy_flurx_ipc_macro::command;
    use serde::Serialize;

    pub fn to_string<O: Serialize>(output: O) -> String {
        serde_json::to_string(&output).expect("Failed to deserialize output value.")
    }
}

/// The common plugin for IPC communication between `Webview` and `bevy`.
pub struct FlurxIpcPlugin;

impl Plugin for FlurxIpcPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxPlugin>() {
            app.add_plugins(FlurxPlugin);
        }

        app.add_plugins((
            FlurxIpcCommandPlugin,
            FlurxIpcEventPlugin
        ));
    }
}