#![allow(clippy::type_complexity)]

use bevy::app::App;
use bevy::prelude::Plugin;
use bevy_flurx::FlurxPlugin;

pub use bevy_flurx_ipc::{command, ipc_handlers};

use crate::bundle::WebViewBundlePlugin;
use crate::plugin::WebviewPlugin;

pub mod plugin;
pub mod bundle;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{
        bundle::*,
        command,
        FlurxWryPlugin,
        ipc_handlers,
    };
}


pub struct FlurxWryPlugin;

impl Plugin for FlurxWryPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxPlugin>() {
            app.add_plugins(FlurxPlugin);
        }

        app.add_plugins((
            WebviewPlugin,
            WebViewBundlePlugin
        ));
    }
}

