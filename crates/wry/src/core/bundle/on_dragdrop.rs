use bevy::prelude::Component;
use wry::DragDropEvent;

pub(crate) type BoxedDragDropEventHandler = Box<dyn Fn(DragDropEvent) -> bool + Send + Sync>;


#[derive(Component, Default)]
pub struct OnDragDrop(Option<BoxedDragDropEventHandler>);

impl OnDragDrop {
    pub const NONE: Self = Self(None);

    pub fn new(f: impl Fn(DragDropEvent) -> bool + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    #[inline]
    pub(crate) fn take(&mut self) -> Option<BoxedDragDropEventHandler> {
        self.0.take()
    }
}


