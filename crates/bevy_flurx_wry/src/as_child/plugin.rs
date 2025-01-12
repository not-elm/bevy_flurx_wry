use crate::as_child::bundle::{Bounds, ParentWindow, Resizable};
use crate::as_child::plugin::grip_zone::GripZonePlugin;
use crate::as_child::plugin::resize::ResizePlugin;
use crate::as_child::CurrentMoving;
use bevy::prelude::{App, Plugin};

mod resize;
mod grip_zone;


/// This is a plugin that uses [`wry::WebViewBuilder::new_as_child`] to spawn a webview as a child of an existing window.
pub(crate) struct AsChildPlugin;

impl Plugin for AsChildPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ParentWindow>()
            .register_type::<Bounds>()
            .register_type::<Resizable>()
            .register_type::<CurrentMoving>()
            .add_plugins((
                ResizePlugin,
                GripZonePlugin,
            ));
    }
}


