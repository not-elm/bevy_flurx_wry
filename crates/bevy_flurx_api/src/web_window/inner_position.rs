use crate::macros::api_plugin;
use crate::web_window::WebWinitWindowParams;
use bevy::prelude::In;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;
use winit::dpi::PhysicalPosition;

api_plugin!(
    /// You'll be able to obtain the window inner position.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const position: PhysicalPosition | null = await window.__FLURX__.Webview.current().innerPosition();
    /// ```
    WebWindowInnerPositionPlugin,
    command: inner_position
);

type Args = String;

#[command(id = "FLURX|web_window::inner_position")]
fn inner_position(In(args): In<Args>) -> Action<Args, Option<PhysicalPosition<i32>>> {
    once::run(system).with(args)
}

fn system(
    In(args): In<Args>,
    web_views: WebWinitWindowParams,
) -> Option<PhysicalPosition<i32>> {
    let window = web_views.winit_window(&args)?;
    window.inner_position().ok()
}