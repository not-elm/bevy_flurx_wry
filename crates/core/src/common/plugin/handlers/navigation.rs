use bevy::app::App;
use bevy::prelude::{Entity, Event, Plugin, Reflect};

use crate::common::plugin::handlers::RegisterWryEvent;


//TODO: Rename
#[derive(Clone, Debug, Event, Reflect)]
pub struct NavigationStarted {
    pub webview_entity: Entity,

    pub uri: String,
}


pub(super) struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_wry_event::<NavigationStarted>();
    }
}