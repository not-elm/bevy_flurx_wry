use bevy::prelude::*;
use crate::common::plugin::WryWebViews;
use crate::prelude::EventEmitter;

pub(crate) struct EventEmitterPlugin;

impl Plugin for EventEmitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, emit);
    }
}

fn emit(
    mut emitters: Query<(Entity, &mut EventEmitter, &Name)>,
    web_views: NonSend<WryWebViews>,
) {
    for (entity, mut emitter, name) in emitters.iter_mut() {
        let Some(webview) = web_views.0.get(&entity) else {
            continue;
        };

        for (event_id, event) in emitter.take_events() {
            let name = name.as_str();
            if let Err(e) = webview.evaluate_script(&format!(
                "window.__FLURX__.__emitEvent('{name}', '{event_id}', {event});"
            )) {
               error!("{e}");
            }
        }
    }
}
