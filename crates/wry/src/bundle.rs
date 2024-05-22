use bevy::prelude::Bundle;

pub use auto_play::AutoPlay;
pub use background::Background;
use bevy_flurx_ipc::prelude::IpcHandlers;
pub use enable_clipboard::EnableClipboard;
pub use event_emitter::EventEmitter;
pub use is_open_devtools::IsOpenDevtools;
pub use on_page_load::{Location, OnPageLoad};
pub use theme::Theme;
pub use uri::Uri;
pub use use_devtools::UseDevtools;
pub use visible::Visible;
pub use user_agent::UserAgent;

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

    /// Custom user-agent for the webview.
    pub user_agent: UserAgent,
    
    /// Represents a webview theme.
    pub theme: Theme,

    /// The handler to process page loading events.
    pub on_page_load: OnPageLoad,

    ///  The ipc invoke handlers.
    pub ipc_handlers: IpcHandlers,

    pub event_emitter: EventEmitter,
}


