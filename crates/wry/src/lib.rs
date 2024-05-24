#![allow(clippy::type_complexity)]


use bevy::app::{App, Plugin};

pub use bevy_flurx_ipc::{command, ipc_handlers};

use crate::as_child::plugin::AsChildPlugin;
use crate::core::plugin::FlurxWryCorePlugin;

pub mod as_child;
pub mod core;


#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{
        command,
        core::prelude::*,
        as_child::prelude::*,
        FlurxWryPlugin,
        ipc_handlers,
    };
}


pub struct FlurxWryPlugin;

impl Plugin for FlurxWryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                FlurxWryCorePlugin,
                AsChildPlugin
            ));
    }
}