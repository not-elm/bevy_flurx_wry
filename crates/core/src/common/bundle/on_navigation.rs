use bevy::prelude::Component;

pub(crate) type BoxedNavigateHandler = Box<dyn Fn(String) -> bool + Send + Sync + 'static>;


#[repr(transparent)]
#[derive(Component, Default)]
pub struct OnNavigation(Option<BoxedNavigateHandler>);

impl OnNavigation {
    pub const NONE: Self = Self(None);

    pub fn new(f: impl Fn(String) -> bool + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    pub(crate) fn take(&mut self) -> Option<BoxedNavigateHandler> {
        self.0.take()
    }
}



