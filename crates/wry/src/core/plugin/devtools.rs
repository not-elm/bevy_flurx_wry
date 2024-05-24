use bevy::app::{App, PostUpdate};
use bevy::prelude::{Commands, Component, DetectChanges, Entity, NonSend, Plugin, Query, Reflect, ReflectComponent};

use crate::core::bundle::{IsOpenDevtools, UseDevtools};
use crate::core::plugin::WebviewMap;

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<DevtoolsReady>()
            .add_systems(PostUpdate, change_open_devtools);
    }
}


#[derive(Component, Reflect)]
#[reflect(Component)]
struct DevtoolsReady;

fn change_open_devtools(
    mut commands: Commands,
    mut views: Query<(Entity, &mut IsOpenDevtools, &UseDevtools, Option<&DevtoolsReady>)>,
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
            commands.entity(entity).insert(DevtoolsReady);
        } else {
            is_open.0 = webview.is_devtools_open();
        }
    }
}