use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

api_plugin!(
    /// You'll be able to get the window is minimizable from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().isMinimizable();
    /// ```
    WebWindowIsResizablePlugin,
    command: is_resizable
);

#[command(id = "FLURX|web_window::is_resizable", internal)]
fn is_resizable(In(args): In<String>) -> Action<String, bool> {
    once::run(system).with(args)
}

fn system(
    In(identifier): In<String>,
    web_views: WebWinitWindowParams,
) -> bool {
    let Some(window) = web_views.winit_window(&identifier) else {
        return false;
    };
    window.is_resizable()
}