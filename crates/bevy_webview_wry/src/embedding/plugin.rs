use crate::embedding::bundle::{Bounds, EmbedWithin, Resizable};
use crate::embedding::plugin::grip_zone::GripZonePlugin;
use crate::embedding::plugin::resize::ResizePlugin;
use crate::embedding::CurrentMoving;
use bevy::prelude::{App, Plugin};

mod resize;
mod grip_zone;


/// This is a plugin that uses [`wry::WebViewBuilder::new_as_child`] to spawn a webview as a child of an existing window.
pub(crate) struct AsChildPlugin;

impl Plugin for AsChildPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<EmbedWithin>()
            .register_type::<Bounds>()
            .register_type::<Resizable>()
            .register_type::<CurrentMoving>()
            .add_plugins((
                ResizePlugin,
                GripZonePlugin,
            ));
    }
}


