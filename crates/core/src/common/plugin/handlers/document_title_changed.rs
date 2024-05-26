//! Controls document title change events.

use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Event, Reflect};

use crate::common::plugin::handlers::RegisterWryEvent;

/// Fired when the document title is changed.
///
/// Please see [`wry::WebViewBuilder::with_document_title_changed_handler`] for detail.
#[derive(Eq, PartialEq, Clone, Event, Reflect)]
pub struct DocumentTitleChanged {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// The new document title.
    pub document_title: String,
}


pub(super) struct DocumentTitlePlugin;

impl Plugin for DocumentTitlePlugin {
    fn build(&self, app: &mut App) {
        app.register_wry_event::<DocumentTitleChanged>();
    }
}

