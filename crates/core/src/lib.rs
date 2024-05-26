#![allow(clippy::type_complexity)]


use std::path::PathBuf;

use bevy::app::{App, Plugin};
use bevy::prelude::{Reflect, Resource};

pub use bevy_flurx_ipc::{command, ipc_handlers};

use crate::as_child::plugin::AsChildPlugin;
use crate::common::plugin::FlurxWryCorePlugin;

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


pub struct FlurxWryPlugin {
    pub content_root: PathBuf,
}

impl Default for FlurxWryPlugin {
    fn default() -> Self {
        Self {
            content_root: PathBuf::from("ui")
        }
    }
}

#[repr(transparent)]
#[derive(Resource, Debug, Reflect, Clone)]
pub(crate) struct WryLocalRoot(pub PathBuf);

impl Plugin for FlurxWryPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<WryLocalRoot>()
            .insert_resource(WryLocalRoot(self.content_root.clone()))
            .add_plugins((
                FlurxWryCorePlugin,
                AsChildPlugin
            ));
    }
}

