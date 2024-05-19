//! Defines a handler that executes the [`Action`](bevy_flurx::prelude::Action) of the command.

use std::any::type_name;

use bevy::prelude::Component;
use bevy::utils::HashMap;
use bevy_flurx::action::Action;
use bevy_flurx::prelude::{ActionSeed, Map, OmitInput};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

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
        me.register(handler)
    }

    /// Add a [`IpcHandler`].
    pub fn register(mut self, handler: impl Into<IpcHandler>) -> Self {
        let handler = handler.into();
        self.0.insert(handler.id.clone(), handler);
        self
    }

    /// Returns the [`ActionSeed`] if exists related to `id`.
    ///
    /// ## Panics
    ///
    /// Panics if args types incorrect.
    pub fn get_action_seed(&self, id: &str, args: Vec<String>) -> Option<ActionSeed<(), String>> {
        self.0.get(id).map(move |handler| {
            (handler.handle)(args)
        })
    }
}

impl From<Vec<IpcHandler>> for IpcHandlers {
    fn from(value: Vec<IpcHandler>) -> Self {
        let mut handlers = Self::default();
        for handler in value {
            handlers = handlers.register(handler);
        }
        handlers
    }
}


/// The ipc invoke handler.
///
/// Usually created via [`invoke_handlers!`](crate::invoke_handlers).
pub struct IpcHandler {
    id: String,
    handle: Box<dyn Fn(Vec<String>) -> ActionSeed<(), String> + Send + Sync>,
}

impl IpcHandler {
    /// Create a new handler.
    ///
    /// The `id` is used when invoking from javascript.
    pub fn new<JsInput, I, O>(
        id: impl Into<String>,
        f: impl Functor<JsInput, I, O> + Send + Sync + 'static,
    ) -> Self
        where
            JsInput: 'static,
            I: 'static,
            O: Serialize + Send + 'static
    {
        Self {
            id: id.into(),
            handle: Box::new(move |args| {
                f.func(args.into_iter())
                    .map(|output| serde_json::to_string(&output).unwrap())
                    .omit_input()
            }),
        }
    }
}

impl<F> From<F> for IpcHandler
    where F: Fn() -> IpcHandler
{
    fn from(f: F) -> Self {
        f()
    }
}


/// Convert the inputs from renderer process to an action.
///
/// This trait has been implemented internally in the library.
pub trait Functor<JsInput, I, O> {
    /// Convert the inputs from renderer process to an action.
    fn func(&self, input: impl Iterator<Item=String>) -> Action<I, O>;
}

macro_rules! impl_functor {
    ($($input: ident,)*) => {
        impl<I, O, F, A, $($input,)* > Functor<($($input,)*), I, O> for F
            where
                F: Fn($($input,)*) -> A,
                A: Into<Action<I, O>>,
                $($input: DeserializeOwned,)*
        {
            #[allow(non_snake_case, unused)]
            fn func(&self, mut input: impl Iterator<Item=String>) -> Action<I, O> {
                (self)($({
                    let arg = serde_json::from_str::<Arg<$input>>(&input.next().unwrap()).expect(&format!("failed deserialize ipc args type: {}", type_name::<$input>()));
                    arg.arg
                },)*).into()
            }
        }
    };
}

#[derive(Deserialize, Debug)]
struct Arg<D> {
    arg: D,
}

impl_functor!();
impl_functor!(I1,);
impl_functor!(I1, I2,);
impl_functor!(I1, I2, I3,);
impl_functor!(I1, I2, I3, I4,);
impl_functor!(I1, I2, I3, I4, I5,);
impl_functor!(I1, I2, I3, I4, I5, I6,);
impl_functor!(I1, I2, I3, I4, I5, I6, I7,);
impl_functor!(I1, I2, I3, I4, I5, I6, I7, I8,);
impl_functor!(I1, I2, I3, I4, I5, I6, I7, I8, I9,);
impl_functor!(I1, I2, I3, I4, I5, I6, I7, I8, I9, I10,);
