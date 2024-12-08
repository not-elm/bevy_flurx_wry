use bevy_ecs::component::Component;
use bevy_window::Window;
use crate::common::bundle::handler::PassedUrl;

pub(crate) type BoxedNewWindowRequest = Box<dyn Fn(PassedUrl) -> Option<Window> + Send + Sync + 'static>;


/// Specifies the callback to be executed on [`wry::WebViewBuilder::with_new_window_req_handler`].
///
/// If [`OnNewWindowRequest::NONE`], the actual callback always returns true.
#[repr(transparent)]
#[derive(Component, Default)]
pub struct OnNewWindowRequest(Option<BoxedNewWindowRequest>);

impl OnNewWindowRequest {
    /// No callback is specified.
    /// 
    /// All new creations of window is permitted.
    pub const NONE: Self = Self(None);

    /// Creates the [`OnNewWindowRequest`].
    pub fn new(f: impl Fn(PassedUrl) -> Option<Window> + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }

    pub(crate) fn take(&mut self) -> Option<BoxedNewWindowRequest>{
        self.0.take()
    }
}


