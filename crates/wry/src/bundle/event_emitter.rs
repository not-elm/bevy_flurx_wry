use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};
use serde::Serialize;

#[derive(Component, Default, Reflect, Debug, Eq, PartialEq, Hash)]
#[reflect(Component, Default)]
pub struct EventEmitter(Vec<(String, String)>);

impl EventEmitter {
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
