//!  Provides the utility apis to read the monitor information. 

mod availables;
mod current;
mod primary;

use bevy_app::{PluginGroup, PluginGroupBuilder};
use serde::Serialize;

pub use crate::monitor::availables::MonitorAvailablesPlugin;
pub use crate::monitor::current::MonitorCurrentPlugin;
pub use crate::monitor::primary::MonitorPrimaryPlugin;

/// Allows  you to use all monitor plugins.
///
/// ## Plugins
///
/// - [MonitorAvailablesPlugin]
/// - [MonitorCurrentPlugin]
/// - [MonitorPrimaryPlugin]
pub struct AllMonitorPlugins;
impl PluginGroup for AllMonitorPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MonitorAvailablesPlugin)
            .add(MonitorCurrentPlugin)
            .add(MonitorPrimaryPlugin)
    }
}

#[derive(Serialize)]
struct PhysicalPosition {
    x: i32,
    y: i32,
}

#[derive(Serialize)]
struct PhysicalSize {
    width: u32,
    height: u32,
}

#[derive(Serialize)]
struct Monitor {
    name: Option<String>,
    position: PhysicalPosition,
    #[serde(rename = "scaleFactor")]
    scale_factor: f64,
    size: PhysicalSize,
}