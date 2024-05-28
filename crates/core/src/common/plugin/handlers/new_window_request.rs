//! Controls the process of creating the new window : [`wry::WebViewBuilder::with_new_window_req_handler`]

use bevy::app::{App, PreUpdate};
use bevy::prelude::{Commands, Entity, Event, EventWriter, Plugin, Reflect, Res};
use bevy::utils::default;
use bevy::window::Window;

use crate::common::plugin::handlers::{RegisterWryEvent, WryEvents};
use crate::prelude::{HandlerUrl, WebviewUri, WryWebViewBundle};


/// The event indicating that a new window has been opened.
///
/// [`OnNewWindowRequest`](crate::prelude::OnNewWindowRequest)
#[derive(Event, Clone, Debug, Reflect)]
pub struct NewWindowOpened {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// The entity associated with the new [`Window`](bevy::prelude::Window).
    pub opened_window_entity: Entity,

    /// The url loaded in new [`Window`](bevy::prelude::Window).
    pub url: HandlerUrl,
}


#[derive(Event, Clone, Debug, Reflect)]
pub(crate) struct NewWindowRequested {
    pub webview_entity: Entity,
    pub url: HandlerUrl,
    pub window: Window,
}


pub(super) struct NewWindowRequestedPlugin;

impl Plugin for NewWindowRequestedPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<NewWindowRequested>()
            .add_event::<NewWindowRequested>()
            .init_resource::<WryEvents<NewWindowRequested>>()
            .register_wry_event::<NewWindowOpened>()
            .add_systems(PreUpdate, open_new_window);
    }
}

fn open_new_window(
    mut commands: Commands,
    mut ew: EventWriter<NewWindowOpened>,
    events: Res<WryEvents<NewWindowRequested>>,
) {
    for request in events.take_events() {
        let opened_window_entity = commands
            .spawn((
                request.window,
                WryWebViewBundle {
                    uri: WebviewUri(request.url.0.to_string()),
                    ..default()
                }
            ))
            .id();

        ew.send(NewWindowOpened {
            webview_entity: request.webview_entity,
            opened_window_entity,
            url: request.url,
        });
    }
}

