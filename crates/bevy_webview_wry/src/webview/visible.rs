use crate::util::WryResultLog;
use crate::webview::WryWebViews;
use bevy::prelude::{App, Changed, Entity, NonSend, Plugin, Query, Update};
use bevy_webview_core::prelude::WebviewVisible;

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
