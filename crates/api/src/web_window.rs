mod title;
mod center;
mod hide;
mod inner_size;
mod is_decorated;

pub use crate::web_window::center::WebWindowCenterPlugin;
pub use crate::web_window::hide::WebWindowHidePlugin;
pub use crate::web_window::title::WebWindowTitlePlugin;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_core::Name;
use bevy_ecs::prelude::{Entity, NonSend, Query};
use bevy_ecs::system::SystemParam;
use bevy_ecs::world::Mut;
use bevy_flurx_wry_core::prelude::ParentWindow;
use bevy_window::{Window, WindowWrapper};
use bevy_winit::WinitWindows;
use crate::web_window::inner_size::WebWindowInnerSizePlugin;
use crate::web_window::is_decorated::WebWindowIsDecoratedPlugin;

/// Allows you to use all window plugins.
///
///## Plugins
///
/// - [WebWindowTitlePlugin]
/// - [WebWindowCenterPlugin]
/// - [WebWindowHidePlugin]
/// - [WebWindowInnerSizePlugin]
/// - [WebWindowIsDecoratedPlugin]
pub struct AllWebWindowPlugins;
impl PluginGroup for AllWebWindowPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WebWindowTitlePlugin)
            .add(WebWindowCenterPlugin)
            .add(WebWindowHidePlugin)
            .add(WebWindowInnerSizePlugin)
            .add(WebWindowIsDecoratedPlugin)
    }
}
#[derive(SystemParam)]
struct WebWinitWindowParams<'w, 's> {
    views: Query<'w, 's, (
        Entity,
        Option<&'static Name>,
        Option<&'static mut Window>,
        Option<&'static ParentWindow>,
    )>,

    windows: NonSend<'w, WinitWindows>,
}

impl WebWinitWindowParams<'_, '_> {
    fn bevy_window_mut(&mut self, identifier: &str) -> Option<Mut<bevy_window::Window>> {
        let entity = self.entity(identifier)?;
        self.views.get_mut(entity).ok().and_then(|query| query.2)
    }

    fn winit_window(&self, identifier: &str) -> Option<&WindowWrapper<winit::window::Window>> {
        let entity = self.entity(identifier)?;
        self.windows.get_window(entity)
    }

    fn entity(&self, identifier: &str) -> Option<Entity> {
        self.views.iter().find_map(|(entity, name, _, parent)| {
            if name.as_ref().map(|n| n.as_str()).unwrap_or_default() != identifier {
                return None;
            }
            if let Some(parent) = parent {
                Some(parent.0)
            } else {
                Some(entity)
            }
        })
    }
}