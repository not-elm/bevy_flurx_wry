//! Defines a handler that executes the [`Action`](bevy_flurx::prelude::Action) of the command.

use std::future::Future;
use std::pin::Pin;

use bevy::app::Update;
use bevy::prelude::{Component, Entity, EventWriter, In, Reflect};
use bevy::utils::HashMap;
use bevy_flurx::action::{Action, once};
use bevy_flurx::task::ReactiveTask;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::ipc_commands::IpcCommand;
use crate::plugin::IpcResolveEvent;

/// The ipc invoke handlers.
///
/// Usually created via [`invoke_handlers!`](crate::invoke_handlers).
#[derive(Component, Default)]
pub struct IpcHandlers(pub(crate) HashMap<String, IpcHandler>);

impl IpcHandlers {
    /// Create a new [`IpcHandlers`].
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use bevy_flurx::prelude::*;
    /// use bevy_flurx_ipc::prelude::*;
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

    /// Returns the [`ActionSeed`] if exists related to `id`.
    ///
    /// ## Panics
    ///
    /// Panics if args types incorrect.
    pub fn get(&self, id: &str) -> Option<IpcFn> {
        self.0.get(id).map(|handler| (handler.handle)())
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

type IpcHandle = Box<dyn Fn() -> IpcFn + Send + Sync>;
type IpcFn = Box<dyn FnOnce(ReactiveTask, IpcCommand) -> IpcFuture>;
type IpcFuture = Pin<Box<dyn Future<Output=()>>>;

/// The ipc invoke handler.
///
/// Usually created via [`ipc_handlers!`](crate::ipc_handlers).
pub struct IpcHandler {
    id: String,
    handle: IpcHandle,
}

impl IpcHandler {
    /// Create a new handler.
    ///
    /// The `id` is used when invoking from javascript.
    pub fn new<Marker>(
        id: impl Into<String>,
        f: impl Functor<Marker> + Send + Sync + 'static,
    ) -> Self
        where
            Marker: 'static
    {
        Self {
            id: id.into(),
            handle: Box::new(move || f.func()),
        }
    }

    /// Returns the ipc-id.
    pub fn id(&self) -> &str{
        &self.id
    }
}

impl<F> From<F> for IpcHandler
    where F: Fn() -> IpcHandler
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
#[derive(Component, Copy, Clone, Reflect)]
pub struct WebviewEntity(pub Entity);


/// Convert the inputs from renderer process to an action.
///
/// This trait has been implemented internally in the library.
pub trait Functor<Marker> {
    /// Convert the inputs from renderer process to an action.
    fn func(&self) -> IpcFn;
}

macro_rules! impl_functor {
    ($($input: ident)?) => {
        #[allow(unused)]
        impl<A, I, O, FI, FO, $($input)?> Functor<(i8, A, I, O, $($input)?)> for FI
            where
                FI: Fn() -> FO,
                FO: Fn($(bevy::prelude::In<$input>,)? WebviewEntity) -> A + 'static,
                A: Into<Action<I, O>> + 'static,
                I: 'static,
                O: Serialize + 'static,
                $($input: DeserializeOwned)?
        {
            fn func(&self) -> IpcFn{
               let f = (self)();
               ipc_action_fn(move |cmd| {
                   f(
                       $(cmd.payload.deserialize_args::<$input>(),)?
                       WebviewEntity(cmd.entity)
                   )
               })
            }
        }

        #[allow(unused)]
        impl<A, I, O, FI, FO, $($input)?> Functor<(i16, A, I, O, $($input)?)> for FI
            where
                FI: Fn() -> FO,
                FO: Fn($(bevy::prelude::In<$input>,)?) -> A + 'static,
                A: Into<Action<I, O>> + 'static,
                I: 'static,
                O: Serialize + 'static,
                $($input: DeserializeOwned)?
        {
            fn func(&self) -> IpcFn{
               let f = (self)();
               ipc_action_fn(move |cmd| {
                   f($(cmd.payload.deserialize_args::<$input>())?)
               })
            }
        }
    };
}

macro_rules! impl_async_functor {
    ($($input: ident)?) => {
        #[allow(unused)]
        impl<Fut, FI, FO, $($input)?> Functor<(u8, $($input)?)> for FI
            where
                FI: Fn() -> FO,
                FO: Fn($(bevy::prelude::In<$input>,)? WebviewEntity, ReactiveTask) -> Fut + 'static,
                Fut: Future,
                Fut::Output: Serialize,
                $($input: DeserializeOwned)?
        {
            fn func(&self) -> IpcFn{
               let f = (self)();
               ipc_fn(move |task, cmd| {
                   f(
                       $(cmd.payload.deserialize_args::<$input>(),)?
                       WebviewEntity(cmd.entity),
                       task,
                   )
               })
            }
        }

        #[allow(unused)]
        impl<Fut, FI, FO, $($input)?> Functor<(u16, $($input)?)> for FI
            where
                FI: Fn() -> FO,
                FO: Fn($(bevy::prelude::In<$input>,)? ReactiveTask) -> Fut + 'static,
                Fut: Future,
                Fut::Output: Serialize,
                $($input: DeserializeOwned)?
        {
            fn func(&self) -> IpcFn{
               let f = (self)();
               ipc_fn(move |task, cmd| {
                   f(
                       $(cmd.payload.deserialize_args::<$input>(),)?
                       task,
                   )
               })
            }
        }

        #[allow(unused)]
        impl<Fut, FI, FO, $($input)?> Functor<(u32, $($input)?)> for FI
            where
                FI: Fn() -> FO,
                FO: Fn($(bevy::prelude::In<$input>)?) -> Fut + 'static,
                Fut: Future,
                Fut::Output: Serialize,
                $($input: DeserializeOwned)?
        {
            fn func(&self) -> IpcFn{
               let f = (self)();
               ipc_fn(move |task, cmd| {
                   f(
                       $(cmd.payload.deserialize_args::<$input>())?
                   )
               })
            }
        }
    };
}



impl_functor!();
impl_functor!(Input);
impl_async_functor!();
impl_async_functor!(Input);


fn ipc_action_fn<I, O, A>(f: impl Fn(IpcCommand) -> A + 'static) -> IpcFn
    where
        A: Into<Action<I, O>> + 'static,
        I: 'static,
        O: Serialize + 'static
{
    ipc_fn(move |task, command| {
        task.will(Update, f(command))
    })
}


fn ipc_fn<Fut>(f: impl Fn(ReactiveTask, IpcCommand) -> Fut + 'static) -> IpcFn
    where
        Fut: Future,
        Fut::Output: Serialize
{
    Box::new(move |task: ReactiveTask, cmd: IpcCommand| Box::pin(async move {
        let entity = cmd.entity;
        let resolve_id = cmd.payload.resolve_id;
        let output = f(task.clone(), cmd).await;
        let output = serde_json::to_string(&output).unwrap();
        task.will(Update, once::run(resolve).with((entity, resolve_id, output))).await;
    }))
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