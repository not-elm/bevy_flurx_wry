//! Controls page loading events.

use bevy::app::App;
use bevy::prelude::{Entity, Event, Plugin, Reflect};

use crate::common::plugin::handlers::RegisterWryEvent;
use crate::prelude::HandlerUrl;

/// Indicates that the content of the page has started loading
#[derive(Event, Clone, Debug, Reflect)]
pub struct PageLoadStarted {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,
    
    /// The url of the page to be loaded.
    pub url: HandlerUrl,
}


/// Indicates that the content of the page has finished loading
#[derive(Event, Clone, Debug, Reflect)]
pub struct PageLoadFinished {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,
   
    /// The url of the loaded page.
    pub url: HandlerUrl,
}


pub(super) struct PageLoadPlugin;

impl Plugin for PageLoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_wry_event::<PageLoadStarted>()
            .register_wry_event::<PageLoadFinished>();
    }
}

