#![allow(clippy::type_complexity)]


pub use bevy_flurx_ipc::{command, ipc_handlers};

pub mod plugin;
pub mod bundle;
mod api;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{
        bundle::*,
        command,
        ipc_handlers,
        plugin::FlurxWryPlugin,
    };
}



