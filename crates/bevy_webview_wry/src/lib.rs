//! Provides the minimum functionality required to display webview.

use crate::embedding::EmbeddingWebviewPlugin;
use bevy::prelude::{App, Plugin, Reflect, Resource};
use bevy_webview_core::bundle::WebViewCoreBundlesPlugin;
use std::path::PathBuf;
use webview::WebviewPlugin;

/// [`bevy_webview_core`]
pub mod core {
    pub use bevy_webview_core::prelude::*;
}

/// [`bevy_flurx_ipc`]
pub mod ipc {
    pub use bevy_flurx_ipc::prelude::*;
}


#[cfg(feature = "api")]
/// [`bevy_flurx_api`]
pub mod api {
    pub use bevy_flurx_api::prelude::*;
}

pub mod embedding;
pub mod webview;
mod util;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{embedding::prelude::*, webview::prelude::*, WebviewWryPlugin};
    #[cfg(feature = "child_window")]
    pub use bevy_child_window::prelude::*;
    #[cfg(feature = "api")]
    pub use bevy_flurx_api::prelude::*;
    pub use bevy_flurx_ipc::prelude::*;
    pub use bevy_webview_core::prelude::*;
}

#[repr(transparent)]
#[derive(Resource, Debug, Reflect, Clone)]
pub(crate) struct WryLocalRoot(pub PathBuf);

/// Provides a mechanism for drawing a webview
/// in a [`Window`](bevy::prelude::Window) using [`wry`].
pub struct WebviewWryPlugin {
    /// Represents the root directory of the local resource.
    /// This value affects [`WebviewUri`](prelude::WebviewUri).
    ///
    /// This directory must be located under the `assets` directory.
    pub local_root: PathBuf,
}

impl Default for WebviewWryPlugin {
    fn default() -> Self {
        Self {
            local_root: PathBuf::from("ui"),
        }
    }
}

impl Plugin for WebviewWryPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<WryLocalRoot>()
            .insert_resource(WryLocalRoot(self.local_root.clone()))
            .add_plugins((
                WebviewPlugin,
                EmbeddingWebviewPlugin,
            ));

        if !app.is_plugin_added::<WebViewCoreBundlesPlugin>() {
            app.add_plugins(WebViewCoreBundlesPlugin);
        }

        #[cfg(feature = "child_window")]
        {
            use bevy_child_window::ChildWindowPlugin;
            if !app.is_plugin_added::<ChildWindowPlugin>() {
                app.add_plugins(ChildWindowPlugin);
            }
        }
    }
}
