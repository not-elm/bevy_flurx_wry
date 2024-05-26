use bevy::app::App;
use bevy::prelude::{Entity, Event, Plugin, Reflect};

use crate::common::plugin::handlers::RegisterWryEvent;

/// Indicates that the content of the page has started loading
#[derive(Event, Clone, Debug, Reflect)]
pub struct PageLoadStarted {
    pub webview_entity: Entity,
    pub uri: String,
}


/// Indicates that the content of the page has started loading
#[derive(Event, Clone, Debug, Reflect)]
pub struct PageLoadFinished {
    pub webview_entity: Entity,
    pub uri: String,
}


pub(super) struct PageLoadPlugin;

impl Plugin for PageLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_wry_event::<PageLoadStarted>()
            .register_wry_event::<PageLoadFinished>();
    }
}

