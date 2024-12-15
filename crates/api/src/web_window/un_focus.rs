use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy_ecs::prelude::In;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

api_plugin!(
    /// You'll be able to un-focus the window from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.Webview.current().unFocus();
    /// ```
    WebWindowUnFocusPlugin,
    command: un_focus
);

#[command(id = "FLURX|web_window::un_focus", internal)]
fn un_focus(In(args): In<String>) -> Action<String> {
    once::run(system).with(args)
}

fn system(
    In(identifier): In<String>,
    mut web_views: WebWinitWindowParams,
) {
    let Some(mut window) = web_views.bevy_window_mut(&identifier) else {
        return;
    };
    window.focused = false;
}