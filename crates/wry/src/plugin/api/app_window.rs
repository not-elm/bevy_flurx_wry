use bevy::prelude::{Entity, In, Query};
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;

use bevy_flurx_ipc::prelude::WebviewEntity;

use crate::prelude::Visible;

pub fn hide(
    WebviewEntity(entity): WebviewEntity
) -> Action<(Entity, bool)> {
    once::run(change_visible).with((entity, false))
}

fn change_visible(
    In((entity, next_visible)): In<(Entity, bool)>,
    mut views: Query<&mut Visible>,
) {
    if let Ok(mut visible) = views.get_mut(entity) {
        if visible.0 != next_visible {
            visible.0 = next_visible;
        }
    }
}