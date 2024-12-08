use bevy_ecs::component::Component;
use wry::DragDropEvent;

pub(crate) type BoxedDragDropEventHandler = Box<dyn Fn(DragDropEvent) -> bool + Send + Sync>;

/// Represents the [`wry::WebViewBuilder::with_drag_drop_handler`].
#[repr(transparent)]
#[derive(Component, Default)]
pub struct OnDragDrop(Option<BoxedDragDropEventHandler>);

impl OnDragDrop {
    /// No callback is specified.
    /// 
    /// The os default behavior of dragdrop does not block it.
    pub const NONE: Self = Self(None);

    /// Create the new [`OnDragDrop`].
    /// 
    /// Return `true` in the callback to block the OS' default behavior.
    pub fn new(f: impl Fn(DragDropEvent) -> bool + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    #[inline]
    pub(crate) fn take(&mut self) -> Option<BoxedDragDropEventHandler> {
        self.0.take()
    }
}


