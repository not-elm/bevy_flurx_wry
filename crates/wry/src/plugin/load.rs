mod protocol;

use std::path::PathBuf;

use bevy::app::{App, Plugin, PreUpdate, Update};
use bevy::prelude::{Commands, Component, Entity, In, NonSend, NonSendMut, Query, Reflect, Res, Window, With, Without, ReflectDefault, ReflectComponent};
use bevy::winit::WinitWindows;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Reactor;
use serde::{Deserialize, Serialize};
use wry::{http, WebView, WebViewBuilder, WebViewBuilderExtWindows};
use wry::http::header::CONTENT_TYPE;
use wry::http::Response;

use bevy_flurx_ipc::ipc_commands::{IpcCommand, IpcCommands};

use crate::bundle::{AutoPlay, Background, EnableClipboard, Theme, Uri, UseDevtools, Visible, };
use crate::plugin::load::protocol::set_protocol;
use crate::plugin::on_page_load::{OnPageArgs, PageLoadEventQueue};
use crate::plugin::WebviewMap;

pub struct LoadWebviewPlugin;

impl Plugin for LoadWebviewPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<WebviewInitialized>()
            .add_systems(PreUpdate, setup_new_windows);
    }
}

#[derive(Component, Default, Reflect, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub(crate) struct WebviewInitialized;

fn setup_new_windows(
    mut commands: Commands,
    views: Query<(
        Entity,
        &Uri,
        &UseDevtools,
        &AutoPlay,
        &EnableClipboard,
        &Visible,
        &Background,
        &Theme,
    ), (Without<WebviewInitialized>, With<Window>)>,
    ipc_commands: Res<IpcCommands>,
    load_queue: Res<PageLoadEventQueue>,
    windows: NonSend<WinitWindows>,
) {
    for (
        entity,
        uri,
        use_devtools,
        auto_play,
        enable_clipboard,
        visible,
        background,
        theme,
    ) in views.iter() {
        let builder = {
            let Some(window) = windows.get_window(entity) else {
                continue;
            };
            let ipc_commands = ipc_commands.clone();
            let load_queue = load_queue.0.clone();
            WebViewBuilder::new_as_child(window)
                .with_initialization_script(include_str!("../../scripts/api.js"))
                .with_devtools(use_devtools.0)
                .with_autoplay(auto_play.0)
                .with_clipboard(enable_clipboard.0)
                .with_visible(visible.0)
                .with_theme(theme.as_wry_theme())
                .with_on_page_load_handler(move |event, uri| {
                    load_queue.lock().unwrap().push(OnPageArgs {
                        event,
                        uri,
                        entity,
                    });
                })
                .with_ipc_handler(move |request| {
                    ipc_commands.push(IpcCommand {
                        entity,
                        payload: serde_json::from_str(request.body()).unwrap(),
                    });
                })
                .with_bounds(wry::Rect{
                    size: wry::dpi::LogicalSize::new(300., 300.).into(),
                    position: wry::dpi::LogicalPosition::new(100., 100.).into()
                })
                .with_url("https://google.com")
        };

        let builder = set_background(builder, background);
        // let builder = set_protocol(builder, uri);

        let webview = builder.build().unwrap();

        commands.entity(entity).insert(WebviewInitialized);
        commands.spawn(Reactor::schedule(move |task| async move {
            task.will(Update, once::run(insert_webview).with((entity, webview))).await;
        }));
    }
}

fn set_background<'a>(builder: WebViewBuilder<'a>, background: &Background) -> WebViewBuilder<'a> {
    match background {
        Background::Unspecified => builder,
        Background::Transparent => builder.with_transparent(true),
        Background::Color(color) => {
            let rgba = color.as_rgba_u8();
            builder.with_background_color((rgba[0], rgba[1], rgba[2], rgba[3]))
        }
    }
}

fn insert_webview(
    In((entity, webview)): In<(Entity, WebView)>,
    mut view_map: NonSendMut<WebviewMap>,
) {
    view_map.0.insert(entity, webview);
}