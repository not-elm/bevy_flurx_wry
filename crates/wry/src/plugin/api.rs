use bevy::app::App;
use bevy::prelude::{Added, Plugin, Query, Update};

use bevy_flurx_ipc::prelude::{IpcHandler, IpcHandlers};

use crate::plugin::create_webview::WebviewInitialized;

mod app_window;
mod app;


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
        h.register(IpcHandler::new("FLURX|app_window::hide", || {
            app_window::hide
        }));

        h.register(IpcHandler::new("FLURX|app::get_name", || {
            app::get_name
        }));
        
        h.register(IpcHandler::new("FLURX|app::get_version", || {
            app::get_version
        }));

        h.register(IpcHandler::new("FLURX|app::exit", || {
            app::exit
        }));
    }
}
