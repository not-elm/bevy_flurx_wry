use bevy::app::App;
use bevy::prelude::{Added, Plugin, Query, Update};

use bevy_flurx_ipc::prelude::IpcHandlers;

use crate::plugin::api::window::plugin_window_show;
use crate::plugin::create_webview::WebviewInitialized;

mod window;

pub struct ApiPlugin;

impl Plugin for ApiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, register_api_handlers);
    }
}

fn register_api_handlers(
    mut handlers: Query<&mut IpcHandlers, Added<WebviewInitialized>>
) {
    for mut h in handlers.iter_mut() {
        h.register(plugin_window_show());
    }
}