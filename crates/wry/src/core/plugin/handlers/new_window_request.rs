use bevy::app::{App, PreUpdate};
use bevy::prelude::{Commands, Entity, Event, EventWriter, Plugin, Reflect, Res};
use bevy::utils::default;
use bevy::window::Window;

use crate::core::plugin::handlers::{RegisterWryEvent, WryEvents};
use crate::prelude::{Uri, WryWebViewBundle};

#[derive(Event, Clone, Debug, Reflect)]
pub struct NewWindowOpened {
    pub webview_entity: Entity,

    pub opened_window_entity: Entity,

    pub url: String,
}


#[derive(Event, Clone, Debug, Reflect)]
pub(crate) struct NewWindowRequested {
    pub webview_entity: Entity,

    pub url: String,

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
                    uri: Uri::Local(request.url.clone()),
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

