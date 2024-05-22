use bevy::app::{App, Plugin};
use bevy::prelude::{Deref, DerefMut, Entity};
use bevy::utils::HashMap;
use bevy_flurx::FlurxPlugin;
use crate::api::ApiAllows;
use crate::as_child::AsChildPlugin;

use crate::bundle::{AutoPlay, Background, EnableClipboard, EventEmitter, HotkeysZoom, HttpsScheme, InitializeFocused, IsOpenDevtools, Theme, Uri, UseDevtools, Visible};
use crate::plugin::api::ApiPlugin;
use crate::plugin::load::LoadWebviewPlugin;
use crate::plugin::devtools::DevtoolsPlugin;
use crate::plugin::event::EventEmitterPlugin;
use crate::plugin::ipc::WryIpcPlugin;
use crate::plugin::on_page_load::OnPageLoadPlugin;
use crate::plugin::visible::VisiblePlugin;
use crate::prelude::Incognito;

mod on_page_load;
mod ipc;
mod devtools;
mod load;
mod event;
mod api;
mod visible;


#[derive(Deref, DerefMut, Default)]
pub(crate) struct WebviewMap(pub HashMap<Entity, wry::WebView>);


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
            .register_type::<InitializeFocused>()
            .register_type::<HotkeysZoom>()
            .register_type::<Incognito>()
            .register_type::<HttpsScheme>()
            .init_resource::<ApiAllows>()
            .add_plugins((
                LoadWebviewPlugin,
                DevtoolsPlugin,
                EventEmitterPlugin,
                OnPageLoadPlugin,
                WryIpcPlugin,
                ApiPlugin,
                AsChildPlugin
            ))
            .add_plugins(VisiblePlugin)
            .init_non_send_resource::<WebviewMap>();
    }
}

