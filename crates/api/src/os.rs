//! 

use bevy_app::{PluginGroup, PluginGroupBuilder};
use crate::macros::api_plugin;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;


/// Allows you to use all os plugins.
///
/// ## Plugins
///
/// - [OsArchPlugin] 
pub struct AllOsPlugins;
impl PluginGroup for AllOsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(OsArchPlugin)
    }
}

api_plugin!(
    /// You'll be able to get a describing the architecture of the CPU from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.os.arch();
    /// ```
    OsArchPlugin,
    command: arch
);

#[command(id = "FLURX|os::arch", internal)]
fn arch() -> Action<(), &'static str> {
    fn f() -> &'static str {
        std::env::consts::ARCH
    }
    once::run(f).with(())
}