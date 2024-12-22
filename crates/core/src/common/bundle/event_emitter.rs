use bevy::prelude::{Component, ReflectComponent};
use bevy::prelude::{ReflectDefault, Reflect};
use serde::Serialize;


/// This component is used to emit events to the webview.
#[repr(transparent)]
#[derive(Component, Default, Reflect, Debug, Eq, PartialEq, Hash)]
#[reflect(Component, Default)]
pub struct EventEmitter(Vec<(String, String)>);

impl EventEmitter {
    /// Emits an event to a webview.
    /// 
    /// On the javascript side, you can receive data by listening to the event as follows: 
    /// `window.__FLURX__.listen("<event_id>", (payload: P) => {})`
    pub fn emit<P>(&mut self, event_id: impl Into<String>, payload: P)
        where
            P: Serialize
    {
        self.0.push((event_id.into(), serde_json::to_string(&payload).unwrap()));
    }
    
    #[inline(always)]
    pub(crate) fn take_events(&mut self) -> Vec<(String, String)>{
        std::mem::take(&mut self.0)
    }
}
