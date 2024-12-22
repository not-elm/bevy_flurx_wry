//! Provides a mechanism to control the basic behavior of Webview.

use crate::common::bundle::{
    AutoPlay, Background, EnableClipboard, EventEmitter, HotkeysZoom, Incognito, InitializeFocused,
    IsOpenDevtools, Theme, UseDevtools, UseHttpsScheme, WebviewUri, WebviewVisible,
};
use crate::common::plugin::event_emitter::EventEmitterPlugin;
use crate::common::plugin::handlers::WryHandlersPlugin;
use crate::common::plugin::ipc_resolve::IpcResolvePlugin;
use crate::common::plugin::load_webview::LoadWebviewPlugin;
use crate::common::plugin::visible::VisiblePlugin;
use crate::common::{WebviewInitialized, WryWebViews};
use crate::prelude::PassedUrl;
use bevy::prelude::{App, Plugin};
use bevy_flurx_ipc::FlurxIpcPlugin;

mod event_emitter;
pub mod handlers;
mod ipc_resolve;
mod load_webview;
mod visible;

#[cfg(debug_assertions)]
mod devtools;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::common::plugin::handlers::prelude::*;
}

pub(crate) struct FlurxWryCommonPlugin;

impl Plugin for FlurxWryCommonPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxIpcPlugin>() {
            app.add_plugins(FlurxIpcPlugin);
        }

        app.register_type::<WebviewInitialized>()
            .register_type::<AutoPlay>()
            .register_type::<Background>()
            .register_type::<EnableClipboard>()
            .register_type::<EventEmitter>()
            .register_type::<WebviewUri>()
            .register_type::<UseDevtools>()
            .register_type::<IsOpenDevtools>()
            .register_type::<WebviewVisible>()
            .register_type::<Theme>()
            .register_type::<InitializeFocused>()
            .register_type::<HotkeysZoom>()
            .register_type::<Incognito>()
            .register_type::<UseHttpsScheme>()
            .register_type::<PassedUrl>()
            .add_plugins((
                LoadWebviewPlugin,
                VisiblePlugin,
                EventEmitterPlugin,
                IpcResolvePlugin,
                WryHandlersPlugin,
            ))
            .init_non_send_resource::<WryWebViews>();

        #[cfg(debug_assertions)]
        app.add_plugins(devtools::DevtoolsPlugin);
    }
}
