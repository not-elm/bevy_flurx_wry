use bevy::app::{App, PostUpdate};
use bevy::prelude::{Commands, Component, DetectChanges, Entity, NonSend, Plugin, Query};

use crate::bundle::{IsOpenDevtools, UseDevtools};
use crate::plugin::WebviewMap;

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, change_open_devtools);
    }
}


#[derive(Component)]
struct DevtoolsInitialized;

fn change_open_devtools(
    mut commands: Commands,
    mut views: Query<(Entity, &mut IsOpenDevtools, &UseDevtools, Option<&DevtoolsInitialized>)>,
    view_map: NonSend<WebviewMap>,
) {
    for (entity, mut is_open, use_devtools, initialized) in views.iter_mut() {
        if !use_devtools.0 {
            is_open.0 = false;
            continue;
        }
        let Some(webview) = view_map.0.get(&entity) else {
            continue;
        };
        if initialized.is_none() || is_open.is_changed() {
            if is_open.0 {
                webview.open_devtools();
            } else {
                webview.close_devtools();
            }
            commands.entity(entity).insert(DevtoolsInitialized);
        } else {
            is_open.0 = webview.is_devtools_open();
        }
    }
}