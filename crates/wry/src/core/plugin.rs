use bevy::app::{App, Plugin};
use bevy_flurx::FlurxPlugin;

use crate::core::{WebviewInitialized, WebviewMap};
use crate::core::bundle::{AutoPlay, Background, EnableClipboard, EventEmitter, HotkeysZoom, HttpsScheme, Incognito, InitializeFocused, IsOpenDevtools, Theme, Uri, UseDevtools, Visible};
use crate::core::plugin::devtools::DevtoolsPlugin;
use crate::core::plugin::event::EventEmitterPlugin;
use crate::core::plugin::ipc::WryIpcPlugin;
use crate::core::plugin::load::LoadWebviewPlugin;
use crate::core::plugin::on_page_load::OnPageLoadPlugin;
use crate::core::plugin::visible::VisiblePlugin;


mod on_page_load;
mod ipc;
mod devtools;
mod load;
mod event;
mod visible;


pub struct FlurxWryCorePlugin;

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
            .register_type::<Visible>()
            .register_type::<Theme>()
            .register_type::<InitializeFocused>()
            .register_type::<HotkeysZoom>()
            .register_type::<Incognito>()
            .register_type::<HttpsScheme>()
            .add_plugins((
                LoadWebviewPlugin,
                DevtoolsPlugin,
                VisiblePlugin,
                EventEmitterPlugin,
                OnPageLoadPlugin,
                WryIpcPlugin,
            ))
            .init_non_send_resource::<WebviewMap>();
    }
}

