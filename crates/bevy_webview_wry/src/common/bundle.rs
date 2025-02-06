//! Declares the webview components.

pub use crate::common::bundle::handler::*;
pub use auto_play::AutoPlay;
pub use background::Background;
use bevy::prelude::Bundle;
use bevy_flurx_ipc::component::IpcHandlers;
pub use browser_accelerator_keys::BrowserAcceleratorKeys;
pub use csp::Csp;
pub use enable_clipboard::EnableClipboard;
pub use event_emitter::EventEmitter;
pub use focused::InitializeFocused;
pub use hotkeys_zoom::HotkeysZoom;
pub use https_scheme::UseHttpsScheme;
pub use incognito::Incognito;
pub use initialization_script::InitializationScripts;
pub use is_open_devtools::IsOpenDevtools;
pub use theme::Theme;
pub use use_devtools::UseDevtools;
pub use user_agent::UserAgent;
pub use visible::WebviewVisible;
pub use webview_uri::{Webview, WebviewUri};

mod auto_play;
mod background;
mod browser_accelerator_keys;
mod enable_clipboard;
mod event_emitter;
mod focused;
mod handler;
mod hotkeys_zoom;
mod https_scheme;
mod incognito;
mod is_open_devtools;
mod theme;
mod use_devtools;
mod user_agent;
mod visible;
mod webview_uri;
mod csp;
mod initialization_script;

/// The following is a list of required components for generating a webview.
///
/// All components defined in this bundle are registered as required components in [`Webview`].
#[derive(Bundle, Default)]
pub struct WebViewBundle {
    /// Represents the display destination of webview.
    pub webview: Webview,

    /// [`wry::WebViewBuilder::with_autoplay`]
    pub auto_play: AutoPlay,

    /// [`wry::WebViewBuilder::with_browser_accelerator_keys`]
    pub browser_accelerator_keys: BrowserAcceleratorKeys,

    /// [`wry::WebViewBuilder::with_clipboard`]
    pub enable_clipboard: EnableClipboard,

    /// [`wry::WebViewBuilder::with_devtools`]
    pub use_devtools: UseDevtools,

    /// Controls opening and closing the webview devtools.
    pub is_open_devtools: IsOpenDevtools,

    /// [`wry::WebViewBuilder::with_visible`]
    pub visible: WebviewVisible,

    /// [`wry::WebViewBuilder::with_background_color`]
    ///
    /// [`wry::WebViewBuilder::with_transparent`]
    pub background: Background,

    /// [`wry::WebViewBuilder::with_user_agent`]
    pub user_agent: UserAgent,

    /// [`wry::WebViewBuilder::with_theme`]
    pub theme: Theme,

    /// [`wry::WebViewBuilder::with_focused`]
    pub initialize_focused: InitializeFocused,

    ///[`wry::WebViewBuilder::with_incognito`]
    pub incognito: Incognito,

    /// [`wry::WebViewBuilder::with_hotkeys_zoom`]
    pub hotkeys_zoom: HotkeysZoom,

    /// [`wry::WebViewBuilder::with_https_scheme`]
    pub use_https_scheme: UseHttpsScheme,

    /// The ipc invoke handlers.
    pub ipc_handlers: IpcHandlers,

    /// [`wry::WebViewBuilder::with_download_started_handler`].
    pub on_download: OnDownload,

    /// [`wry::WebViewBuilder::with_drag_drop_handler`].
    pub on_dragdrop: OnDragDrop,

    /// [`wry::WebViewBuilder::with_navigation_handler`]
    pub on_navigation: OnNavigation,

    /// [`wry::WebViewBuilder::with_new_window_req_handler`]
    pub on_new_window_request: OnNewWindowRequest,

    /// Used to emit events to the webview.
    pub event_emitter: EventEmitter,
}

