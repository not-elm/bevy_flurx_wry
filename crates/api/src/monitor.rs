mod available_monitors;
mod current_monitor;
mod primary;

pub use crate::monitor::available_monitors::WindowAvailableMonitorsPlugin;
pub use crate::monitor::current_monitor::WindowCurrentMonitorsPlugin;
pub use crate::monitor::primary::MonitorPrimaryPlugin;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use serde::Serialize;

/// Allows  you to use all window plugins.
///
/// ## Plugins
///
/// - [WindowAvailableMonitorsPlugin]
/// - [WindowCurrentMonitorsPlugin]
/// - [MonitorPrimaryPlugin]
pub struct AllWindowPlugins;
impl PluginGroup for AllWindowPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WindowAvailableMonitorsPlugin)
            .add(WindowCurrentMonitorsPlugin)
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