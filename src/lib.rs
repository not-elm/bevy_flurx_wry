#![cfg_attr(doc_cfg, feature(doc_cfg))]

//! This crate provides a mechanism to create a webview based on `wry`.

/// Provides the minimum functionality required to display webview.
pub mod core {
    pub use bevy_flurx_wry_core::*;
}

/// Provides the ipc mechanisms.
pub mod ipc {
    pub use bevy_flurx_ipc::*;
}

#[allow(missing_docs)]
pub mod prelude {
    pub use bevy_flurx_ipc::prelude::*;
    pub use bevy_flurx_wry_core::prelude::*;
}