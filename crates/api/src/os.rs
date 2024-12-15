//! Provides the utility actions to read a system information.

mod family;

use crate::macros::api_plugin;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

pub use crate::os::family::OsFamilyPlugin;

/// Allows you to use all os plugins.
///
/// ## Plugins
///
/// - [OsArchPlugin]
/// - [OsFamilyPlugin]
pub struct AllOsPlugins;
impl PluginGroup for AllOsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(OsArchPlugin)
            .add(OsFamilyPlugin)
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