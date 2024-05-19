use bevy::prelude::Component;
use bevy_flurx::action::Action;
use bevy_flurx::prelude::{ActionSeed, Omit};
use serde::{Deserialize, Serialize};
use wry::PageLoadEvent;


/// The loaded page location.
#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Default, Debug, Hash, Serialize, Deserialize)]
pub struct Location(pub String);


/// The handler to process page loading events.
#[derive(Component, Default)]
pub struct OnPageLoad(pub(crate) Option<Box<dyn Fn(PageLoadEvent, String) -> ActionSeed + Send + Sync>>);

impl OnPageLoad {
    pub fn make<I, O, A>(f: impl Fn(PageLoadEvent, Location) -> A + Send + Sync + 'static) -> Self
        where
            I: 'static,
            O: 'static,
            A: Into<Action<I, O>>
    {
        Self(Some(Box::new(move |event, uri| {
            f(event, Location(uri)).into().omit()
        })))
    }
}
