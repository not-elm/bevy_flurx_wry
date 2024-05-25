use bevy::app::{App, Plugin};

use crate::as_child::bundle::{Bounds, ParentWindow, Resizable};
use crate::as_child::CurrentMoving;
use crate::as_child::plugin::resize::ResizePlugin;
use crate::as_child::plugin::toolbar::ToolbarPlugin;

mod resize;
mod toolbar;
mod api;

pub struct AsChildPlugin;

impl Plugin for AsChildPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ParentWindow>()
            .register_type::<Bounds>()
            .register_type::<Resizable>()
            .register_type::<CurrentMoving>()
            .add_plugins((
                ResizePlugin,
                ToolbarPlugin,
                // ApiPlugin
            ));
    }
}


