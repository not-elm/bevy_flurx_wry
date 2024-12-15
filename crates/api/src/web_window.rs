//! Provides the mechanism to control the window from the webview.

mod title;
mod center;
mod hide;
mod inner_size;
mod is_decorated;
mod is_focused;
mod is_fullscreen;
mod is_maximized;
mod is_maximizable;
mod is_minimizable;
mod is_minimized;
mod is_resizable;
mod is_visible;
mod maximize;
mod minimize;
mod show;
mod set_decorations;
mod set_window_mode;
mod focus;
mod un_focus;
mod set_cursor_hit_test;
mod un_maximize;
mod un_minimize;

pub use crate::web_window::center::WebWindowCenterPlugin;
use crate::web_window::focus::WebWindowFocusPlugin;
pub use crate::web_window::hide::WebWindowHidePlugin;
use crate::web_window::inner_size::WebWindowInnerSizePlugin;
use crate::web_window::is_decorated::WebWindowIsDecoratedPlugin;
use crate::web_window::is_focused::WebWindowIsFocusedPlugin;
use crate::web_window::is_fullscreen::WebWindowIsFullscreenPlugin;
use crate::web_window::is_maximizable::WebWindowIsMaximizablePlugin;
use crate::web_window::is_maximized::WebWindowIsMaximizedPlugin;
use crate::web_window::is_minimizable::WebWindowIsMinimizablePlugin;
use crate::web_window::is_minimized::WebWindowIsMinimizedPlugin;
use crate::web_window::is_resizable::WebWindowIsResizablePlugin;
use crate::web_window::is_visible::WebWindowIsVisiblePlugin;
use crate::web_window::maximize::WebWindowMaximizePlugin;
use crate::web_window::minimize::WebWindowMinimizePlugin;
use crate::web_window::set_cursor_hit_test::WebWindowSetCursorHitTestPlugin;
use crate::web_window::set_decorations::WebWindowSetDecorationsPlugin;
use crate::web_window::set_window_mode::WebWindowSetWindowModePlugin;
use crate::web_window::show::WebWindowShowPlugin;
pub use crate::web_window::title::WebWindowTitlePlugin;
use crate::web_window::un_focus::WebWindowUnFocusPlugin;
use crate::web_window::un_maximize::WebWindowUnMaximizePlugin;
use crate::web_window::un_minimize::WebWindowUnMinimizePlugin;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_core::Name;
use bevy_ecs::prelude::{Entity, NonSend, Query};
use bevy_ecs::system::SystemParam;
use bevy_ecs::world::Mut;
use bevy_flurx_wry_core::prelude::ParentWindow;
use bevy_window::{Window, WindowWrapper};
use bevy_winit::WinitWindows;

/// Allows you to use all window plugins.
///
///## Plugins
///
/// - [WebWindowTitlePlugin]
/// - [WebWindowCenterPlugin]
/// - [WebWindowHidePlugin]
/// - [WebWindowShowPlugin]
/// - [WebWindowInnerSizePlugin]
/// - [WebWindowIsDecoratedPlugin]
/// - [WebWindowIsFocusedPlugin]
/// - [WebWindowIsFullscreenPlugin]
/// - [WebWindowIsMaximizedPlugin]
/// - [WebWindowIsMaximizablePlugin]
/// - [WebWindowIsMinimizablePlugin]
/// - [WebWindowIsMinimizedPlugin]
/// - [WebWindowIsResizablePlugin]
/// - [WebWindowIsVisiblePlugin]
/// - [WebWindowMaximizePlugin]
/// - [WebWindowUnMaximizePlugin]
/// - [WebWindowMinimizePlugin]
/// - [WebWindowUnMinimizePlugin]
/// - [WebWindowSetDecorationsPlugin]
/// - [WebWindowSetWindowModePlugin]
/// - [WebWindowFocusPlugin]
/// - [WebWindowUnFocusPlugin]
/// - [WebWindowSetCursorHitTestPlugin]
pub struct AllWebWindowPlugins;
impl PluginGroup for AllWebWindowPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(WebWindowTitlePlugin)
            .add(WebWindowCenterPlugin)
            .add(WebWindowHidePlugin)
            .add(WebWindowShowPlugin)
            .add(WebWindowInnerSizePlugin)
            .add(WebWindowIsDecoratedPlugin)
            .add(WebWindowIsFocusedPlugin)
            .add(WebWindowIsFullscreenPlugin)
            .add(WebWindowIsMaximizedPlugin)
            .add(WebWindowIsMaximizablePlugin)
            .add(WebWindowIsMinimizablePlugin)
            .add(WebWindowIsMinimizedPlugin)
            .add(WebWindowIsResizablePlugin)
            .add(WebWindowIsVisiblePlugin)
            .add(WebWindowMaximizePlugin)
            .add(WebWindowUnMaximizePlugin)
            .add(WebWindowMinimizePlugin)
            .add(WebWindowUnMinimizePlugin)
            .add(WebWindowSetDecorationsPlugin)
            .add(WebWindowSetWindowModePlugin)
            .add(WebWindowFocusPlugin)
            .add(WebWindowUnFocusPlugin)
            .add(WebWindowSetCursorHitTestPlugin)
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