use bevy::app::{App, Plugin, PreUpdate};
use bevy::prelude::{Commands, Entity, NonSend, NonSendMut, Or, Query, Res, Window, With, Without};
use bevy::winit::WinitWindows;
use wry::WebViewBuilder;

use crate::as_child::bundle::{Bounds, ParentWindow};
use crate::common::bundle::{AutoPlay, Background, BrowserAcceleratorKeys, EnableClipboard, HotkeysZoom, Incognito, InitializeFocused, Theme, UseDevtools, UseHttpsScheme, UserAgent, WebviewUri, WebviewVisible};
use crate::common::plugin::handlers::{HandlerQueries, WryEventParams};
use crate::common::plugin::load::ipc::IpcHandlerParams;
use crate::common::plugin::load::protocol::feed_uri;
use crate::common::plugin::WryWebViews;
use crate::common::WebviewInitialized;
use crate::WryLocalRoot;

mod protocol;
mod ipc;

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
    &'a Incognito,
);

type Configs2<'a> = (
    &'a InitializeFocused,
    &'a HotkeysZoom,
    &'a UserAgent,
    &'a WebviewUri,
);

type ConfigsPlatformSpecific<'a> = (
    &'a Theme,
    &'a BrowserAcceleratorKeys,
    &'a UseHttpsScheme
);

fn setup_new_windows(
    mut commands: Commands,
    mut web_views: NonSendMut<WryWebViews>,
    mut views: Query<(
        Entity,
        HandlerQueries,
        Configs1,
        Configs2,
        ConfigsPlatformSpecific,
        Option<&ParentWindow>,
        Option<&Bounds>
    ), (Without<WebviewInitialized>, Or<(With<Window>, With<ParentWindow>)>)>,
    ipc_params: IpcHandlerParams,
    event_params: WryEventParams,
    local_root: Res<WryLocalRoot>,
    windows: NonSend<WinitWindows>,
) {
    for (
        webview_entity,
        handlers,
        configs1,
        configs2,
        configs_platform,
        parent_window,
        bounds
    ) in views.iter_mut() {
        let Some(builder) = new_builder(webview_entity, &parent_window, &bounds, &windows) else {
            continue;
        };

        let builder = ipc_params.feed_ipc(webview_entity, builder);
        let builder = event_params.feed_handlers(webview_entity, handlers, builder);
        let builder = feed_configs1(builder, configs1);
        let builder = feed_configs2(builder, configs2, &local_root);
        let builder = feed_configs_platform_configs(builder, configs_platform);
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
        incognito,
    ): Configs1,
) -> WebViewBuilder<'a> {
    let builder = builder
        .with_devtools(dev_tools.0)
        .with_autoplay(auto_play.0)
        .with_clipboard(enable_clipboard.0)
        .with_visible(visible.0)
        .with_incognito(incognito.0);

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
            include_str!("../../../scripts/gripZone.js")
        ));

    if let Some(user_agent) = user_agent.0.as_ref() {
        builder = builder.with_user_agent(user_agent);
    }

    feed_uri(builder, uri, local_root)
}

#[allow(clippy::needless_return, unreachable_code)]
fn feed_configs_platform_configs<'a>(
    builder: WebViewBuilder<'a>,
    (
        theme,
        browser_accelerator_keys,
        https_scheme
    ): ConfigsPlatformSpecific,
) -> WebViewBuilder<'a> {
    #[cfg(target_os = "windows")]
    {
        use wry::WebViewBuilderExtWindows;
        return builder
            .with_theme(theme.as_wry_theme())
            .with_browser_accelerator_keys(browser_accelerator_keys.0)
            .with_https_scheme(https_scheme.0);
    }
    #[cfg(target_os = "android")]
    {
        use wry::WebViewBuilderExtAndroid;
        return builder.with_https_scheme(https_scheme.0);
    }
    return builder;
}


