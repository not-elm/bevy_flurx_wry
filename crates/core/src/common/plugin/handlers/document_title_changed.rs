use bevy::app::{App, Plugin, };
use bevy::prelude::{Entity, Event, Reflect};
use crate::common::plugin::handlers::RegisterWryEvent;

#[derive(Eq, PartialEq, Clone, Event, Reflect)]
pub struct DocumentTitleChanged {
    pub entity: Entity,

    pub document_title: String,
}


pub(super) struct DocumentTitlePlugin;

impl Plugin for DocumentTitlePlugin {
    fn build(&self, app: &mut App) {
        app.register_wry_event::<DocumentTitleChanged>();
    }
}

