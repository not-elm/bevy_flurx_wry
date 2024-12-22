use bevy::prelude::In;
use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

api_plugin!(
    /// You'll be able to get a focus state from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().isFocused();
    /// ```
    WebWindowIsFocusedPlugin,
    command: is_focused
);

#[command(id = "FLURX|web_window::is_focused", internal)]
fn is_focused(In(args): In<String>) -> Action<String, bool> {
    once::run(system).with(args)
}

fn system(
    In(identifier): In<String>,
    web_views: WebWinitWindowParams,
) -> bool {
    let Some(window) = web_views.winit_window(&identifier) else {
        return false;
    };
    window.has_focus()
}