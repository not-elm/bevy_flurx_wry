use bevy::app::{App, Plugin};
use bevy::prelude::Bundle;
use bevy_flurx_ipc::prelude::IpcHandlers;

pub use auto_play::AutoPlay;
pub use background::Background;
pub use enable_clipboard::EnableClipboard;
pub use is_open_devtools::IsOpenDevtools;
pub use on_page_load::{Location, OnPageLoad};
pub use theme::Theme;
pub use uri::Uri;
pub use use_devtools::UseDevtools;
pub use visible::Visible;
pub use webview_uninitialized::WebviewUninitialized;

mod auto_play;
mod background;
mod enable_clipboard;
mod on_page_load;
mod theme;
mod use_devtools;
mod webview_uninitialized;
mod visible;
mod uri;
mod is_open_devtools;


#[derive(Bundle, Default)]
pub struct WryWebViewBundle {
    /// Represents the display destination of webview.
    pub uri: Uri,

    /// Represents whether the webview devtools should be used.
    pub use_devtools: UseDevtools,

    /// Represents whether the webview devtools should be used.
    pub is_open_devtools: IsOpenDevtools,

    /// Represents whether all media can be played without user interaction.
    pub auto_play: AutoPlay,

    /// Represents whether enables clipboard access for the page rendered.
    pub enable_clipboard: EnableClipboard,

    /// Represents whether the webview should be visible. 
    pub visible: Visible,

    /// Represents the webview background. 
    pub background: Background,

    /// Represents a webview theme.
    pub theme: Theme,

    /// The handler to process page loading events.
    pub on_page_load: OnPageLoad,

    ///  The ipc invoke handlers.
    pub ipc_handlers: IpcHandlers,

    /// This is a marker component that indicates a webview is in a uninitialized state.
    pub request_initialize: WebviewUninitialized,
}


/// Plugin related to [`WryWebViewBundle`].
///
/// This plugin registers components that implement [`Reflect`](bevy::prelude::Reflect).
pub struct WebViewBundlePlugin;

impl Plugin for WebViewBundlePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<AutoPlay>()
            .register_type::<Background>()
            .register_type::<EnableClipboard>()
            .register_type::<Uri>()
            .register_type::<UseDevtools>()
            .register_type::<IsOpenDevtools>()
            .register_type::<Visible>()
            .register_type::<Theme>()
            .register_type::<WebviewUninitialized>();
    }
}