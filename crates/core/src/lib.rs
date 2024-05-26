//! Provides the minimum functionality required to display webview.

#![allow(clippy::type_complexity)]

use std::path::PathBuf;

use bevy::app::{App, Plugin};
use bevy::prelude::{Reflect, Resource};

pub use bevy_flurx_ipc::{command, ipc_handlers};

use crate::as_child::plugin::AsChildPlugin;
use crate::common::plugin::FlurxWryCommonPlugin;

pub mod as_child;
pub mod common;


#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{
        as_child::prelude::*,
        command,
        common::prelude::*,
        FlurxWryPlugin,
        ipc_handlers,
    };
}


#[repr(transparent)]
#[derive(Resource, Debug, Reflect, Clone)]
pub(crate) struct WryLocalRoot(pub PathBuf);


/// Provides a mechanism for drawing a webview 
/// in a [`Window`](bevy::prelude::Window) using [`wry`].
pub struct FlurxWryPlugin {
    /// Represents the root directory of the local resource.
    /// This value affects [`Uri::Local`](crate::prelude::Uri::Local).
    /// 
    /// This directory must be located under the `assets` directory.
    pub local_root: PathBuf,
}

impl Default for FlurxWryPlugin {
    fn default() -> Self {
        Self {
            local_root: PathBuf::from("ui")
        }
    }
}

impl Plugin for FlurxWryPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<WryLocalRoot>()
            .insert_resource(WryLocalRoot(self.local_root.clone()))
            .add_plugins((
                FlurxWryCommonPlugin,
                AsChildPlugin
            ));
    }
}

