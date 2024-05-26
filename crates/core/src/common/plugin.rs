use bevy::app::{App, Plugin};
use bevy_flurx::FlurxPlugin;

use crate::common::{WebviewInitialized, WryWebViews};
use crate::common::bundle::{AutoPlay, Background, EnableClipboard, EventEmitter, HotkeysZoom, UseHttpsScheme, Incognito, InitializeFocused, IsOpenDevtools, Theme, Uri, UseDevtools, WebviewVisible};
use crate::common::plugin::devtools::DevtoolsPlugin;
use crate::common::plugin::event_emitter::EventEmitterPlugin;
use crate::common::plugin::handlers::WryHandlersPlugin;
use crate::common::plugin::ipc_resolve::IpcResolvePlugin;
use crate::common::plugin::load::LoadWebviewPlugin;
use crate::common::plugin::visible::VisiblePlugin;

mod ipc_resolve;
mod devtools;
mod load;
mod event_emitter;
mod visible;
pub mod handlers;

pub mod prelude{
    pub use crate::common::plugin::{
        handlers::prelude::*
    };
}

pub(crate) struct FlurxWryCorePlugin;

impl Plugin for FlurxWryCorePlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxPlugin>() {
            app.add_plugins(FlurxPlugin);
        }

        app
            .register_type::<WebviewInitialized>()
            .register_type::<AutoPlay>()
            .register_type::<Background>()
            .register_type::<EnableClipboard>()
            .register_type::<EventEmitter>()
            .register_type::<Uri>()
            .register_type::<UseDevtools>()
            .register_type::<IsOpenDevtools>()
            .register_type::<WebviewVisible>()
            .register_type::<Theme>()
            .register_type::<InitializeFocused>()
            .register_type::<HotkeysZoom>()
            .register_type::<Incognito>()
            .register_type::<UseHttpsScheme>()
            .add_plugins((
                LoadWebviewPlugin,
                DevtoolsPlugin,
                VisiblePlugin,
                EventEmitterPlugin,
                IpcResolvePlugin,
                WryHandlersPlugin
            ))
            .init_non_send_resource::<WryWebViews>();
    }
}

