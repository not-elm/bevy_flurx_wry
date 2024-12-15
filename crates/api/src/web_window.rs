mod title;

pub use crate::web_window::title::WebWindowTitlePlugin;
use bevy_core::Name;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs::prelude::{Entity, NonSend, Query};
use bevy_ecs::system::SystemParam;
use bevy_flurx_wry_core::prelude::ParentWindow;
use bevy_window::WindowWrapper;
use bevy_winit::WinitWindows;

/// Allows you to use all window plugins.
///
///## Plugins
///
/// - [WebWindowTitlePlugin]
pub struct AllWebWindowPlugins;
impl PluginGroup for AllWebWindowPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WebWindowTitlePlugin)
    }
}

#[derive(SystemParam)]
struct WebWindowParams<'w, 's> {
    parent: Query<'w, 's, (Entity, &'static Name, Option<&'static ParentWindow>)>,
    web_views: NonSend<'w, WinitWindows>,
}

impl WebWindowParams<'_, '_> {
    fn winit_window(&self, identifier: &str) -> Option<&WindowWrapper<winit::window::Window>> {
        let entity = self.parent.iter().find_map(|(entity, name, parent)| {
            if name.as_str() != identifier {
                return None;
            }
            if let Some(parent) = parent {
                Some(parent.0)
            } else {
                Some(entity)
            }
        })?;
        self.web_views.get_window(entity)
    }
}