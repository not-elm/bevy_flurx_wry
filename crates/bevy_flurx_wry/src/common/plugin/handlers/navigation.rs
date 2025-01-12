//! Controls navigation events.

use crate::common::plugin::handlers::RegisterWryEvent;
use crate::prelude::PassedUrl;
use bevy::prelude::{Entity, Event, App, Plugin, Reflect};

/// The event is fired when [`OnNavigation`](crate::prelude::OnNavigation) returns `true`.
#[derive(Clone, Debug, Event, Reflect)]
pub struct Navigated {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// URL of the navigation destination
    pub uri: PassedUrl,
}

pub(super) struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_wry_event::<Navigated>();
    }
}
