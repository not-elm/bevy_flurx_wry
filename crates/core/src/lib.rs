//! Provides the minimum functionality required to display webview.

use crate::as_child::plugin::AsChildPlugin;
use crate::common::plugin::FlurxWryCommonPlugin;
use bevy_app::{App, Plugin};
use bevy_ecs::prelude::Resource;
use bevy_reflect::Reflect;
use std::path::PathBuf;

pub mod as_child;
pub mod common;
mod util;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{as_child::prelude::*, common::prelude::*, FlurxWryPlugin};
}

#[repr(transparent)]
#[derive(Resource, Debug, Reflect, Clone)]
pub(crate) struct WryLocalRoot(pub PathBuf);

/// Provides a mechanism for drawing a webview
/// in a [`Window`](bevy::prelude::Window) using [`wry`].
pub struct FlurxWryPlugin {
    /// Represents the root directory of the local resource.
    /// This value affects [`WebviewUri`](crate::prelude::WebviewUri).
    ///
    /// This directory must be located under the `assets` directory.
    pub local_root: PathBuf,
}

impl Default for FlurxWryPlugin {
    fn default() -> Self {
        Self {
            local_root: PathBuf::from("ui"),
        }
    }
}

impl Plugin for FlurxWryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WryLocalRoot>()
            .insert_resource(WryLocalRoot(self.local_root.clone()))
            .add_plugins((FlurxWryCommonPlugin, AsChildPlugin));
    }
}
