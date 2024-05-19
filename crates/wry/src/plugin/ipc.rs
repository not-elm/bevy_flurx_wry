use bevy::app::{App, Plugin, Update};
use bevy::prelude::{EventReader, NonSendMut};

use bevy_flurx_ipc::plugin::{FlurxIpcPlugin, IpcResolveEvent};

use crate::plugin::WebviewMap;

pub struct WryIpcPlugin;

impl Plugin for WryIpcPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxIpcPlugin>() {
            app.add_plugins(FlurxIpcPlugin);
        }
        
        app.add_systems(Update, resolve_event);
    }
}

fn resolve_event(
    mut er: EventReader<IpcResolveEvent>,
    mut views: NonSendMut<WebviewMap>,
) {
    for IpcResolveEvent {
        entity,
        resolve_id,
        output
    } in er.read() {
        if let Some(view) = views.get_mut(entity) {
            view.evaluate_script(&format!("window.__FLURX__.resolveIpc({resolve_id}, {output})")).unwrap();
        }
    }
}