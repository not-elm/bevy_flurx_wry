//! This library facilitates communication between the `webview` and `bevy`.


use bevy::app::{App, Plugin};
use bevy_flurx::FlurxPlugin;

pub use bevy_flurx_ipc_macro::command;

use crate::ipc_commands::FlurxIpcCommandPlugin;
use crate::prelude::FlurxIpcEventPlugin;

pub mod component;
pub mod ipc_commands;
pub mod ipc_events;


#[allow(missing_docs)]
pub mod prelude {
    pub use bevy_flurx_ipc_macro::command;

    pub use crate::{
        component::*,
        FlurxIpcPlugin,
        ipc_commands::*,
        ipc_events::*,
        ipc_handlers,
    };
}

/// Create the [`IpcHandlers`](crate::prelude::IpcHandlers) from the commands.
///
/// ```no_run
///
/// use bevy_flurx::action::once;
/// use bevy_flurx::prelude::ActionSeed;
/// use bevy_flurx_ipc::prelude::*;
///
/// #[command]
/// fn ipc_command() -> ActionSeed{
///     once::run(||{
///
///     })
/// }
///
/// ipc_handlers![
///     ipc_command
/// ];
/// ```
#[macro_export]
macro_rules! ipc_handlers {
    () => ($crate::prelude::IpcHandlers::default());
    ($functor: expr $(,$others: expr)* $(,)?) => ({
        $crate::prelude::IpcHandlers::new($functor)
            $(.with($others))*
    });
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