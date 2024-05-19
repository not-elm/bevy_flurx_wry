//! This library facilitates communication between the `webview` and `bevy`.


pub use bevy_flurx_ipc_macro::command;

pub mod component;
pub mod plugin;
pub mod ipc_command_queue;


#[allow(missing_docs)]
pub mod prelude{
    pub use bevy_flurx_ipc_macro::command;
    pub use crate::{
        ipc_handlers,
        component::*,
        plugin::{IpcResolveEvent, FlurxIpcPlugin},
        ipc_command_queue::IpcCommandQueue
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
            $(.register($others))*
    });
}

