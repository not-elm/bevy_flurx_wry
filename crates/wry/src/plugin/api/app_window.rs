
use bevy::prelude::{Entity, In, NonSend, Query};
use bevy::window::Window;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;

use bevy_flurx_ipc::prelude::WebviewEntity;
use crate::as_child::ParentWindow;
use crate::plugin::WebviewMap;

pub fn hide(
    WebviewEntity(entity): WebviewEntity
) -> Action<(Entity, bool)> {
    once::run(change_visible).with((entity, false))
}

fn change_visible(
    In((entity, visible)): In<(Entity, bool)>,
    mut window: Query<&mut Window>,
    views: NonSend<WebviewMap>,
    view: Query<Option<&ParentWindow>>
){
    if view.get(entity).is_ok_and(|parent|parent.is_some()){
        if let Some(webview) = views.0.get(&entity){
            webview.set_visible(visible).unwrap();
        }
    }else if let Ok(mut window) = window.get_mut(entity){
        window.visible = visible;
    }
}