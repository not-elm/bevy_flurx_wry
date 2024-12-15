use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy_ecs::prelude::In;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

api_plugin!(
    /// You'll be able to maximize a window from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().maximize();
    /// ```
    WebWindowMaximizePlugin,
    command: maximize
);

#[command(id = "FLURX|web_window::maximize", internal)]
fn maximize(In(args): In<String>) -> Action<String> {
    once::run(system).with(args)
}

fn system(
    In(identifier): In<String>,
    web_views: WebWinitWindowParams,
)  {
    let Some(window) = web_views.winit_window(&identifier) else {
        return;
    };
    window.set_maximized(true);
}