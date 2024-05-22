use bevy::app::{App, Plugin, PreUpdate, Update};
use bevy::prelude::{Commands, Component, Entity, In, NonSend, NonSendMut, Or, Query, Reflect, ReflectComponent, ReflectDefault, Res, Window, With, Without};
use bevy::winit::WinitWindows;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Reactor;
use serde::{Deserialize, Serialize};
use wry::{WebView, WebViewBuilder, WebViewBuilderExtWindows};

use bevy_flurx_ipc::ipc_commands::{IpcCommand, IpcCommands};

use crate::as_child::{Bounds, ParentWindow};
use crate::bundle::{AutoPlay, Background, EnableClipboard, Theme, Uri, UseDevtools, UserAgent, Visible};
use crate::plugin::load::protocol::set_protocol;
use crate::plugin::on_page_load::{OnPageArgs, PageLoadEventQueue};
use crate::plugin::WebviewMap;

mod protocol;

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
        &UserAgent,
        Option<&ParentWindow>,
        Option<&Bounds>
    ), (Without<WebviewInitialized>, Or<(With<Window>, With<ParentWindow>)>)>,
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
        user_agent,
        parent_window,
        bounds
    ) in views.iter() {
        let builder = {
            let Some(builder) = new_builder(entity, &parent_window, &bounds, &windows) else {
                continue;
            };
            let ipc_commands = ipc_commands.clone();
            let load_queue = load_queue.0.clone();
            builder
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
        };

        let builder = set_user_agent(builder, user_agent);
        let builder = set_background(builder, background);
        let builder = set_protocol(builder, uri);

        let webview = builder.build().unwrap();
        if let Some(bounds) = bounds {
            // For some reason, `WebViewBuilder::with_bounds` alone doesn't render
            webview.set_bounds(bounds.as_wry_rect()).unwrap();
        }

        commands.entity(entity).insert(WebviewInitialized);
        commands.spawn(Reactor::schedule(move |task| async move {
            task.will(Update, once::run(insert_webview).with((entity, webview))).await;
        }));
    }
}

fn new_builder<'a>(
    entity: Entity,
    parent_window: &Option<&ParentWindow>,
    bounds: &Option<&Bounds>,
    windows: &'a WinitWindows,
) -> Option<WebViewBuilder<'a>> {
    if let Some(ParentWindow(parent_entity)) = parent_window {
        let mut builder = WebViewBuilder::new_as_child(windows.get_window(*parent_entity)?);
        if let Some(bounds) = bounds {
            builder = builder.with_bounds(bounds.as_wry_rect());
        }
        Some(builder)
    } else {
        Some(WebViewBuilder::new(windows.get_window(entity)?))
    }
}

fn set_user_agent<'a>(builder: WebViewBuilder<'a>, user_agent: &UserAgent) -> WebViewBuilder<'a>{
    if let Some(user_agent) = user_agent.0.as_ref(){
        builder.with_user_agent(user_agent)
    }else{
        builder
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