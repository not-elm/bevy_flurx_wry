//! Provides the minimum functionality required to display webview.

use crate::common::plugin::FlurxWryCommonPlugin;
use crate::embedding::plugin::AsChildPlugin;
use bevy::prelude::{App, Plugin, Reflect, Resource};
use std::path::PathBuf;

pub mod embedding;
pub mod common;
mod util;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{common::prelude::*, embedding::prelude::*, WebviewWryPlugin};
}

#[repr(transparent)]
#[derive(Resource, Debug, Reflect, Clone)]
pub(crate) struct WryLocalRoot(pub PathBuf);

/// Provides a mechanism for drawing a webview
/// in a [`Window`](bevy::prelude::Window) using [`wry`].
pub struct WebviewWryPlugin {
    /// Represents the root directory of the local resource.
    /// This value affects [`WebviewUri`](crate::prelude::WebviewUri).
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
            .add_plugins((FlurxWryCommonPlugin, AsChildPlugin));
    }
}
