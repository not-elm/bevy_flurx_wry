use crate::common::bundle::WebviewVisible;
use crate::common::plugin::WryWebViews;
use crate::util::WryResultLog;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Changed, NonSend, Query};

pub struct VisiblePlugin;

impl Plugin for VisiblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, change_visible);
    }
}

fn change_visible(
    view_map: NonSend<WryWebViews>,
    views: Query<(Entity, &WebviewVisible), Changed<WebviewVisible>>,
) {
    for (entity, visible) in views.iter() {
        if let Some(webview) = view_map.0.get(&entity) {
            webview.set_visible(visible.0).output_log_if_failed();
        }
    }
}
