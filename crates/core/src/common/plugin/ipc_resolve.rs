use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::{EventReader, NonSendMut};

use bevy_flurx_ipc::prelude::IpcResolveEvent;

use crate::common::plugin::WryWebViews;

pub struct IpcResolvePlugin;

impl Plugin for IpcResolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, resolve_event);
    }
}

fn resolve_event(
    mut er: EventReader<IpcResolveEvent>,
    mut views: NonSendMut<WryWebViews>,
) {
    for IpcResolveEvent {
        entity,
        resolve_id,
        output
    } in er.read() {
        if let Some(view) = views.get_mut(entity) {
            view.evaluate_script(&format!("window.__FLURX__.__resolveIpc({resolve_id}, {output})")).unwrap();
        }
    }
}

