use bevy::app::App;
use bevy::prelude::{Added, Plugin, Query, Res, Update};

use bevy_flurx_ipc::prelude::{IpcHandler, IpcHandlers};

use crate::api::{ApiAllows, AppApiAllows};
use crate::plugin::load::WebviewInitialized;

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
    mut handlers: Query<&mut IpcHandlers, Added<WebviewInitialized>>,
    allows: Res<ApiAllows>,
) {
    for mut h in handlers.iter_mut() {
        h.register(IpcHandler::new("FLURX|app_window::hide", || {
            app_window::hide
        }));
        
        setup_app(&mut h, &allows.app);
    }
}

fn setup_app(
    handler: &mut IpcHandlers,
    app: &AppApiAllows,
) {
    if app.get_name {
        handler.register(IpcHandler::new("FLURX|app::get_name", || {
            app::get_name
        }));
    }

    if app.get_version {
        handler.register(IpcHandler::new("FLURX|app::get_version", || {
            app::get_version
        }));
    }

    if app.exit {
        handler.register(IpcHandler::new("FLURX|app::exit", || {
            app::exit
        }));
    }
}