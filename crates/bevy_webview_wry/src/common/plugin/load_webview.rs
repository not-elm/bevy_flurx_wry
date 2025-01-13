use crate::common::bundle::{
    AutoPlay, Background, BrowserAcceleratorKeys, EnableClipboard, HotkeysZoom, Incognito,
    InitializeFocused, Theme, UseDevtools, UseHttpsScheme, UserAgent, WebviewUri, WebviewVisible,
};
use crate::common::plugin::handlers::{HandlerQueries, WryEventParams};
use crate::common::plugin::load_webview::ipc::IpcHandlerParams;
use crate::common::plugin::load_webview::protocol::feed_uri;
use crate::common::plugin::WryWebViews;
use crate::common::WebviewInitialized;
use crate::embedding::bundle::{Bounds, ParentWindow};
use crate::prelude::Csp;
use crate::prelude::InitializationScripts;
use crate::WryLocalRoot;
use bevy::prelude::{App, Commands, Entity, Name, NonSend, NonSendMut, Or, Plugin, PreUpdate, Query, Res, Window, With, Without};
use bevy::winit::WinitWindows;
use rand::distributions::DistString;
use std::ops::Deref;
#[cfg(target_os = "macos")]
use wry::WebViewExtMacOS;
use wry::{WebView, WebViewBuilder};

mod ipc;
mod protocol;

pub struct LoadWebviewPlugin;

impl Plugin for LoadWebviewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, load_web_views);

        #[cfg(target_os = "macos")]
        {
            use bevy::prelude::IntoSystemConfigs;
            app.add_systems(PreUpdate, (
                resize_webview_inner_window.run_if(bevy::prelude::on_event::<bevy::window::WindowResized>),
            ));
        }
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
    &'a InitializationScripts,
    Option<&'a Csp>,
    Option<&'a Name>,
);

type ConfigsPlatformSpecific<'a> = (&'a Theme, &'a BrowserAcceleratorKeys, &'a UseHttpsScheme);

fn load_web_views(
    mut commands: Commands,
    mut web_views: NonSendMut<WryWebViews>,
    mut views: Query<
        (
            Entity,
            HandlerQueries,
            Configs1,
            Configs2,
            ConfigsPlatformSpecific,
            Option<&ParentWindow>,
            Option<&Bounds>,
        ),
        (
            Without<WebviewInitialized>,
            Or<(With<Window>, With<ParentWindow>)>,
        ),
    >,
    ipc_params: IpcHandlerParams,
    event_params: WryEventParams,
    local_root: Res<WryLocalRoot>,
    windows: NonSend<WinitWindows>,
) {
    for (webview_entity, handlers, configs1, configs2, configs_platform, parent_window, bounds) in
        views.iter_mut()
    {
        let Some(builder) = new_builder(parent_window.is_some(), &bounds) else {
            continue;
        };

        let builder = ipc_params.feed_ipc(webview_entity, builder);
        let builder = event_params.feed_handlers(webview_entity, handlers, builder);
        let builder = feed_configs1(builder, configs1);
        let builder = feed_configs2(
            builder,
            &mut commands,
            webview_entity,
            configs2,
            &local_root,
        );
        let builder = feed_platform_configs(builder, configs_platform);
        let Some(Ok(webview)) = build_webview(builder, webview_entity, parent_window, &windows)
        else {
            continue;
        };
        #[cfg(target_os = "macos")]
        // Safety: Ensure that attach the winit window to webview.
        unsafe {
            if parent_window.is_none() {
                attach_inner_window(&webview.ns_window(), &webview.webview());
            }
        }
        commands
            .entity(webview_entity)
            .insert(WebviewInitialized(()));
        web_views.0.insert(webview_entity, webview);
    }
}

fn new_builder<'a>(has_parent: bool, bounds: &Option<&Bounds>) -> Option<WebViewBuilder<'a>> {
    if has_parent {
        let mut builder = WebViewBuilder::new();
        if let Some(bounds) = bounds {
            builder = builder.with_bounds(bounds.as_wry_rect());
        }
        Some(builder)
    } else {
        Some(WebViewBuilder::new())
    }
}

fn feed_configs1<'a>(
    builder: WebViewBuilder<'a>,
    (dev_tools, auto_play, enable_clipboard, visible, background, incognito): Configs1,
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
            use bevy::prelude::ColorToPacked;
            let rgba = color.to_srgba().to_u8_array();
            builder.with_background_color((rgba[0], rgba[1], rgba[2], rgba[3]))
        }
    }
}

fn feed_configs2<'a>(
    builder: WebViewBuilder<'a>,
    commands: &mut Commands,
    entity: Entity,
    (focused, hotkeys_zoom, user_agent, uri, initialization_scripts, csp, name): Configs2,
    local_root: &WryLocalRoot,
) -> WebViewBuilder<'a> {
    let identifier = if let Some(name) = name {
        name.to_string()
    } else {
        let mut rng = rand::thread_rng();
        let random_code = rand::distributions::Alphanumeric.sample_string(&mut rng, 32);
        commands
            .entity(entity)
            .insert(Name::new(random_code.clone()));
        random_code
    };
    let mut builder = builder
        .with_focused(focused.0)
        .with_hotkeys_zoom(hotkeys_zoom.0)
        .with_initialization_script(&format!(
            "{};{};{};{}",
            include_str!("../../../scripts/api.js"),
            include_str!("../../../scripts/gripZone.js"),
            include_str!("../../../scripts/windowIdentifier.js")
                .replace("<WINDOW_IDENTIFIER>", &identifier),
            initialization_scripts.to_scripts(),
        ));
    if let Some(user_agent) = user_agent.0.as_ref() {
        builder = builder.with_user_agent(user_agent);
    }

    feed_uri(builder, uri, local_root, csp.cloned())
}

#[allow(clippy::needless_return, unreachable_code, unused_variables)]
fn feed_platform_configs<'a>(
    builder: WebViewBuilder<'a>,
    (theme, browser_accelerator_keys, https_scheme): ConfigsPlatformSpecific,
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

fn build_webview(
    builder: WebViewBuilder,
    window_entity: Entity,
    parent_window: Option<&ParentWindow>,
    windows: &WinitWindows,
) -> Option<wry::Result<WebView>> {
    if let Some(parent_window) = parent_window
        .map(|parent| parent.0)
        .and_then(|parent| windows.get_window(parent))
    {
        Some(builder.build_as_child(parent_window.deref()))
    } else if cfg!(target_os = "macos") {
        windows
            .get_window(window_entity)
            .map(|window| builder.build_as_child(window.deref()))
    } else {
        windows
            .get_window(window_entity)
            .map(|window| builder.build(window.deref()))
    }
}

#[cfg(target_os = "macos")]
unsafe fn attach_inner_window(
    application_window: &objc2_app_kit::NSWindow,
    webview: &wry::WryWebView,
) {
    use objc2_app_kit::{NSApplication, NSAutoresizingMaskOptions};

    webview.removeFromSuperview();
    webview.setAutoresizingMask(
        NSAutoresizingMaskOptions::NSViewHeightSizable
            | NSAutoresizingMaskOptions::NSViewWidthSizable,
    );
    let mtw = objc2_foundation::MainThreadMarker::new().unwrap();
    let inner_window = objc2_app_kit::NSPanel::new(mtw);
    inner_window.setTitle(&objc2_foundation::NSString::from_str(""));
    inner_window.setStyleMask(
        objc2_app_kit::NSWindowStyleMask::HUDWindow |
            objc2_app_kit::NSWindowStyleMask::Titled |
            objc2_app_kit::NSWindowStyleMask::NonactivatingPanel |
            objc2_app_kit::NSWindowStyleMask::FullSizeContentView
    );
    inner_window.setMovable(false);
    inner_window.makeFirstResponder(Some(webview));

    let content_rect = application_window.contentRectForFrameRect(application_window.frame());
    inner_window.setFrame_display(content_rect, true);
    inner_window.setHidesOnDeactivate(false);
    inner_window.setTitlebarAppearsTransparent(true);
    inner_window.setTitleVisibility(objc2_app_kit::NSWindowTitleVisibility::NSWindowTitleHidden);

    inner_window.setContentView(Some(webview));

    inner_window.becomeKeyWindow();
    inner_window.makeFirstResponder(Some(webview));

    application_window.addChildWindow_ordered(&inner_window, objc2_app_kit::NSWindowOrderingMode::NSWindowAbove);
    application_window.makeFirstResponder(Some(&inner_window));

    inner_window.makeKeyAndOrderFront(None);

    let app = NSApplication::sharedApplication(mtw);
    if objc2_foundation::NSProcessInfo::processInfo().operatingSystemVersion().majorVersion >= 14 {
        NSApplication::activate(&app);
    } else {
        #[allow(deprecated)]
        NSApplication::activateIgnoringOtherApps(&app, true);
    }
}

#[cfg(target_os = "macos")]
fn resize_webview_inner_window(
    mut er: bevy::prelude::EventReader<bevy::window::WindowResized>,
    winit_windows: NonSend<WinitWindows>,
    wry_web_views: NonSend<WryWebViews>,
) {
    #[allow(deprecated)]
    use wry::raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
    for e in er.read() {
        let Some(winit_window) = winit_windows.get_window(e.window) else {
            continue;
        };
        let Some(wry_webview) = wry_web_views.0.get(&e.window) else {
            continue;
        };
        #[allow(deprecated)]
        let Ok(RawWindowHandle::AppKit(handle)) = winit_window.raw_window_handle() else {
            continue;
        };
        let ns_view = handle.ns_view.as_ptr();
        // SAFETY: The pointer came from `WindowHandle`, which ensures
        // that the `AppKitWindowHandle` contains a valid pointer to an
        // `NSView`.
        // Unwrap is fine, since the pointer came from `NonNull`.
        let ns_view: objc2::rc::Id<objc2_app_kit::NSView> = unsafe { objc2::rc::Id::retain(ns_view.cast()) }.unwrap();
        let Some(ns_window) = ns_view.window() else {
            continue;
        };
        wry_webview.ns_window().setFrame_display(ns_window.contentRectForFrameRect(ns_window.frame()), true);
    }
}

