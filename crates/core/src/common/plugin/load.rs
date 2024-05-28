use bevy::app::{App, Plugin, PreUpdate};
use bevy::prelude::{Commands, Entity, NonSend, NonSendMut, Or, Query, Res, Window, With, Without};
use bevy::winit::WinitWindows;
use wry::{WebViewBuilder, WebViewBuilderExtWindows};

use bevy_flurx_ipc::ipc_commands::{IpcCommand, IpcCommands};

use crate::as_child::bundle::{Bounds, ParentWindow};
use crate::common::bundle::{AutoPlay, Background, BrowserAcceleratorKeys, EnableClipboard, HotkeysZoom, Incognito, InitializeFocused, Theme, UseDevtools, UseHttpsScheme, UserAgent, WebviewUri, WebviewVisible};
use crate::common::plugin::handlers::{HandlerQueries, WryEventParams};
use crate::common::plugin::load::protocol::feed_uri;
use crate::common::plugin::WryWebViews;
use crate::common::WebviewInitialized;
use crate::WryLocalRoot;

mod protocol;

pub struct LoadWebviewPlugin;

impl Plugin for LoadWebviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, setup_new_windows);
    }
}


type Configs1<'a> = (
    &'a UseDevtools,
    &'a AutoPlay,
    &'a EnableClipboard,
    &'a WebviewVisible,
    &'a Background,
    &'a Theme,
    &'a Incognito,
    &'a BrowserAcceleratorKeys,
    &'a UseHttpsScheme
);

type Configs2<'a> = (
    &'a InitializeFocused,
    &'a HotkeysZoom,
    &'a UserAgent,
    &'a WebviewUri,
);

fn setup_new_windows(
    mut commands: Commands,
    mut web_views: NonSendMut<WryWebViews>,
    mut views: Query<(
        Entity,
        HandlerQueries,
        Configs1,
        Configs2,
        Option<&ParentWindow>,
        Option<&Bounds>
    ), (Without<WebviewInitialized>, Or<(With<Window>, With<ParentWindow>)>)>,
    ipc_commands: Res<IpcCommands>,
    event_params: WryEventParams,
    local_root: Res<WryLocalRoot>,
    windows: NonSend<WinitWindows>,
) {
    for (
        webview_entity,
        handlers,
        configs1,
        configs2,
        parent_window,
        bounds
    ) in views.iter_mut() {
        let Some(builder) = new_builder(webview_entity, &parent_window, &bounds, &windows) else {
            continue;
        };
        let ipc_commands = ipc_commands.clone();
        let builder = builder.with_ipc_handler(move |request| {
            ipc_commands.push(IpcCommand {
                entity: webview_entity,
                payload: serde_json::from_str(request.body()).unwrap(),
            });
        });

        let builder = event_params.feed_handlers(webview_entity, handlers, builder);
        let builder = feed_configs1(builder, configs1);
        let builder = feed_configs2(builder, configs2, &local_root);
        let webview = builder.build().unwrap();
        commands.entity(webview_entity).insert(WebviewInitialized(()));
        web_views.0.insert(webview_entity, webview);
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

fn feed_configs1<'a>(
    builder: WebViewBuilder<'a>,
    (
        dev_tools,
        auto_play,
        enable_clipboard,
        visible,
        background,
        theme,
        incognito,
        browser_accelerator_keys,
        https_scheme
    ): Configs1,
) -> WebViewBuilder<'a> {
    let builder = builder
        .with_devtools(dev_tools.0)
        .with_autoplay(auto_play.0)
        .with_clipboard(enable_clipboard.0)
        .with_visible(visible.0)
        .with_theme(theme.as_wry_theme())
        .with_incognito(incognito.0)
        .with_browser_accelerator_keys(browser_accelerator_keys.0)
        .with_https_scheme(https_scheme.0);

    match background {
        Background::Unspecified => builder,
        Background::Transparent => builder.with_transparent(true),
        Background::Color(color) => {
            let rgba = color.as_rgba_u8();
            builder.with_background_color((rgba[0], rgba[1], rgba[2], rgba[3]))
        }
    }
}

fn feed_configs2<'a>(
    builder: WebViewBuilder<'a>,
    (
        focused,
        hotkeys_zoom,
        user_agent,
        uri,
    ): Configs2,
    local_root: &WryLocalRoot,
) -> WebViewBuilder<'a> {
    let mut builder = builder
        .with_focused(focused.0)
        .with_hotkeys_zoom(hotkeys_zoom.0)
        .with_initialization_script(&format!(
            "{}{}",
            include_str!("../../../scripts/api.js"),
            include_str!("./toolbar.js")
        ));

    if let Some(user_agent) = user_agent.0.as_ref() {
        builder = builder.with_user_agent(user_agent);
    }

    feed_uri(builder, uri, local_root)
}
