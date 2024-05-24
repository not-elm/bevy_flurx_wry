use bevy::prelude::Component;
use bevy::window::Window;

pub(crate) type BoxedNewWindowRequest = Box<dyn Fn(String) -> Option<Window> + Send + Sync + 'static>;


#[repr(transparent)]
#[derive(Component, Default)]
pub struct OnNewWindowRequest(Option<BoxedNewWindowRequest>);

impl OnNewWindowRequest {
    pub const NONE: Self = Self(None);

    pub fn new(f: impl Fn(String) -> Option<Window> + Send + Sync + 'static) -> Self {
        Self(Some(Box::new(f)))
    }
    
    
    pub(crate) fn take(&mut self) -> Option<BoxedNewWindowRequest>{
        self.0.take()
    }
}


