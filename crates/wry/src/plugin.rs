use bevy::app::{App, Plugin};
use bevy::prelude::{Deref, DerefMut, Entity};
use bevy::utils::HashMap;
use bevy_flurx::FlurxPlugin;

use crate::bundle::{AutoPlay, Background, EnableClipboard, EventEmitter, IsOpenDevtools, Theme, Uri, UseDevtools, Visible};
use crate::plugin::api::ApiPlugin;
use crate::plugin::create_webview::CreateWebviewPlugin;
use crate::plugin::devtools::DevtoolsPlugin;
use crate::plugin::event::EventEmitterPlugin;
use crate::plugin::ipc::WryIpcPlugin;
use crate::plugin::on_page_load::OnPageLoadPlugin;

mod on_page_load;
mod ipc;
mod devtools;
mod create_webview;
mod event;
mod api;


#[derive(Deref, DerefMut, Default)]
struct WebviewMap(HashMap<Entity, wry::WebView>);


pub struct FlurxWryPlugin;

impl Plugin for FlurxWryPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxPlugin>() {
            app.add_plugins(FlurxPlugin);
        }

        app
            .register_type::<AutoPlay>()
            .register_type::<Background>()
            .register_type::<EnableClipboard>()
            .register_type::<EventEmitter>()
            .register_type::<Uri>()
            .register_type::<UseDevtools>()
            .register_type::<IsOpenDevtools>()
            .register_type::<Visible>()
            .register_type::<Theme>()
            .add_plugins((
                CreateWebviewPlugin,
                DevtoolsPlugin,
                EventEmitterPlugin,
                OnPageLoadPlugin,
                WryIpcPlugin,
                ApiPlugin
            ))
            .init_non_send_resource::<WebviewMap>();
    }
}

