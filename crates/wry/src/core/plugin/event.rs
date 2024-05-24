use bevy::app::{App, PostUpdate};
use bevy::prelude::{Entity, NonSend, Plugin, Query};

use crate::core::bundle::EventEmitter;
use crate::core::plugin::WebviewMap;

pub(crate) struct EventEmitterPlugin;

impl Plugin for EventEmitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, emit);
    }
}

fn emit(
    mut emitters: Query<(Entity, &mut EventEmitter)>,
    webview_map: NonSend<WebviewMap>,
) {
    for (entity, mut emitter) in emitters.iter_mut() {
        let Some(webview) = webview_map.0.get(&entity) else {
            continue;
        };
        for (event_id, event) in emitter.take_events() {
            webview.evaluate_script(&format!("window.__FLURX__.emitEvent('{event_id}', {event});")).unwrap();
        }
    }
}
