use bevy::prelude::Bundle;

pub use auto_play::AutoPlay;
pub use background::Background;
use bevy_flurx_ipc::prelude::IpcHandlers;
pub use enable_clipboard::EnableClipboard;
pub use event_emitter::EventEmitter;
pub use focused::InitializeFocused;
pub use hotkeys_zoom::HotkeysZoom;
pub use incognito::Incognito;
pub use is_open_devtools::IsOpenDevtools;
pub use on_page_load::{Location, OnPageLoad};
pub use theme::Theme;
pub use uri::Uri;
pub use use_devtools::UseDevtools;
pub use user_agent::UserAgent;
pub use visible::Visible;
pub use browser_accelerator_keys::BrowserAcceleratorKeys;
pub use https_scheme::HttpsScheme;

mod auto_play;
mod background;
mod enable_clipboard;
mod on_page_load;
mod theme;
mod use_devtools;
mod visible;
mod uri;
mod is_open_devtools;
mod event_emitter;
mod user_agent;
mod focused;
mod hotkeys_zoom;
mod incognito;
mod browser_accelerator_keys;
mod https_scheme;


#[derive(Bundle, Default)]
pub struct WryWebViewBundle {
    /// [`wry::WebViewBuilder::with_autoplay`]
    pub auto_play: AutoPlay,

    /// [`wry::WebViewBuilder::with_browser_accelerator_keys`]
    pub browser_accelerator_keys: BrowserAcceleratorKeys,

    /// [`wry::WebViewBuilder::with_clipboard`]
    pub enable_clipboard: EnableClipboard,

    /// Represents the display destination of webview.
    pub uri: Uri,

    /// [`wry::WebViewBuilder::with_devtools`]
    pub use_devtools: UseDevtools,

    /// Represents whether the webview devtools should be used.
    pub is_open_devtools: IsOpenDevtools,

    /// [`wry::WebViewBuilder::with_visible`]
    pub visible: Visible,

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
    pub https_scheme: HttpsScheme,

    /// [`wry::WebViewBuilder::with_on_page_load_handler`]
    pub on_page_load: OnPageLoad,

    /// The ipc invoke handlers.
    pub ipc_handlers: IpcHandlers,

    pub event_emitter: EventEmitter,
}

