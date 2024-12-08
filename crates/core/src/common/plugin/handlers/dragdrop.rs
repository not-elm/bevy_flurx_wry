//! Controls dragdrop events.

use crate::common::plugin::handlers::WryEvents;
use bevy_app::{App, Plugin, PreUpdate};
use bevy_math::IVec2;
use bevy_ecs::prelude::{Entity, Event, EventWriter,Res};
use std::path::PathBuf;
use bevy_reflect::Reflect;
use wry::DragDropEvent;

/// Fired when a file or other item is dragged into the Webview.
#[derive(Event, Clone, Reflect, Debug)]
pub struct DragEntered {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// List of paths that are being dragged onto the webview.
    pub paths: Vec<PathBuf>,

    /// Position of the drag operation, relative to the webview top-left corner.
    pub position: IVec2,
}

/// Fired while a file or other item is being dragged over to the web view.
#[derive(Event, Clone, Reflect, Debug)]
pub struct DragOver {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// Position of the drag operation, relative to the webview top-left corner.
    pub position: IVec2,
}

/// Fired when a file or other item is dropped onto the Webview.
#[derive(Event, Clone, Reflect, Debug)]
pub struct Dropped {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// List of paths that are being dropped onto the window.
    pub paths: Vec<PathBuf>,

    /// Position of the drag operation, relative to the webview top-left corner.
    pub position: IVec2,
}

/// Fired when the item under drag left the webview.
#[derive(Event, Copy, Clone, Reflect, Debug)]
pub struct DragLeft {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,
}

pub(crate) struct WryDragDrop {
    pub webview_entity: Entity,
    pub event: DragDropEvent,
}

pub(super) struct DragDropPlugin;

impl Plugin for DragDropPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DragEntered>()
            .add_event::<DragEntered>()
            .register_type::<DragOver>()
            .add_event::<DragOver>()
            .register_type::<Dropped>()
            .add_event::<Dropped>()
            .register_type::<DragLeft>()
            .add_event::<DragLeft>()
            .init_resource::<WryEvents<WryDragDrop>>()
            .add_systems(PreUpdate, send_dragdrop_event);
    }
}

fn send_dragdrop_event(
    mut entered: EventWriter<DragEntered>,
    mut over: EventWriter<DragOver>,
    mut dropped: EventWriter<Dropped>,
    mut leaved: EventWriter<DragLeft>,
    events: Res<WryEvents<WryDragDrop>>,
) {
    for event in events.take_events() {
        let webview_entity = event.webview_entity;
        match event.event {
            DragDropEvent::Enter { paths, position } => {
                entered.send(DragEntered {
                    webview_entity,
                    paths,
                    position: IVec2::new(position.0, position.1),
                });
            }
            DragDropEvent::Over { position } => {
                over.send(DragOver {
                    webview_entity,
                    position: IVec2::new(position.0, position.1),
                });
            }
            DragDropEvent::Drop { paths, position } => {
                dropped.send(Dropped {
                    webview_entity,
                    paths,
                    position: IVec2::new(position.0, position.1),
                });
            }
            DragDropEvent::Leave => {
                leaved.send(DragLeft { webview_entity });
            }
            _ => {}
        }
    }
}
