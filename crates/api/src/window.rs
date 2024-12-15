mod available_monitors;

pub use crate::window::available_monitors::WindowAvailableMonitorsPlugin;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use serde::Serialize;


/// Allows  you to use all window plugins.
///
/// ## Plugins
///
/// - [WindowAvailableMonitorsPlugin]
pub struct AllWindowPlugins;
impl PluginGroup for AllWindowPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WindowAvailableMonitorsPlugin)
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
