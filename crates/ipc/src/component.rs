//! Defines a handler that executes the [`Action`] of the command.

use crate::ipc_commands::IpcCommand;
use bevy_ecs::prelude::{Component, Entity};
use bevy_ecs::system::Commands;
use bevy_reflect::Reflect;
use bevy_utils::HashMap;

/// The ipc invoke handlers.
///
/// Usually created via [`ipc_handlers!`](crate::ipc_handlers).
#[repr(transparent)]
#[derive(Component, Default)]
pub struct IpcHandlers(pub(crate) HashMap<String, IpcHandler>);

impl IpcHandlers {
    /// Create a new [`IpcHandlers`].
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use bevy_flurx::prelude::*;
    /// use bevy_flurx_wry::prelude::*;
    ///
    /// #[command]
    /// fn hello() -> ActionSeed<(), String>{
    ///     once::run(||{
    ///         "hello world".to_string()
    ///     })
    /// }
    ///
    /// IpcHandlers::new(hello);
    /// ```
    pub fn new(handler: impl Into<IpcHandler>) -> Self {
        let me = Self::default();
        me.with(handler)
    }

    /// Add a [`IpcHandler`].
    pub fn with(mut self, handler: impl Into<IpcHandler>) -> Self {
        self.register(handler);
        self
    }

    /// Add a [`IpcHandler`].
    pub fn register(&mut self, handler: impl Into<IpcHandler>) {
        let handler = handler.into();
        self.0.insert(handler.id.clone(), handler);
    }

    /// Returns the function that creates the future if exists related to `id`.
    pub fn get(&self, id: &str) -> Option<IpcFn> {
        self.0.get(id).map(|handler| handler.f)
    }
}

impl From<Vec<IpcHandler>> for IpcHandlers {
    fn from(value: Vec<IpcHandler>) -> Self {
        let mut handlers = Self::default();
        for handler in value {
            handlers.register(handler);
        }
        handlers
    }
}

/// The function created by command macro.
pub type IpcFn = fn(commands: &mut Commands, IpcCommand);

/// The ipc invoke handler.
///
/// Usually created via [`ipc_handlers!`](crate::ipc_handlers).
pub struct IpcHandler {
    id: String,
    f: IpcFn,
}

impl IpcHandler {
    /// Create a new handler.
    ///
    /// The `id` is used when invoking from javascript.
    pub fn new(
        id: impl Into<String>,
        f: IpcFn,
    ) -> Self {
        Self {
            id: id.into(),
            f,
        }
    }

    /// Returns the ipc-id.
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl<F> From<F> for IpcHandler
where
    F: Fn() -> IpcHandler,
{
    fn from(f: F) -> Self {
        f()
    }
}

/// This is one of the optional arguments passed to the ipc command.
///
/// It represents the entity associated with the `Webview` components
/// such as [`IpcHandler`].
#[repr(transparent)]
#[derive(Component, Copy, Clone, Reflect, Debug, Eq, PartialEq)]
pub struct WebviewEntity(pub Entity);
