#![doc=include_str!("../README.md")]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

/// Provides the minimum functionality required to display webview.
pub mod core {
    pub use bevy_flurx_wry_core::*;
}

/// Provides the ipc mechanisms.
pub mod ipc {
    pub use bevy_flurx_ipc::*;
}

/// Provides the apis.
pub mod api{
    pub use bevy_flurx_wry_api::*;
}

#[allow(missing_docs)]
pub mod prelude {
    pub use bevy_flurx_ipc::prelude::*;
    pub use bevy_flurx_wry_core::prelude::*;
    pub use bevy_flurx_wry_api::prelude::*;
}