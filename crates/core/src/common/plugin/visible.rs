use bevy::app::{App, Plugin};
use bevy::prelude::{Changed, Entity, NonSend, Query, Update};

use crate::common::bundle::WebviewVisible;
use crate::common::plugin::WryWebViews;

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
            webview.set_visible(visible.0).unwrap();
        }
    }
}