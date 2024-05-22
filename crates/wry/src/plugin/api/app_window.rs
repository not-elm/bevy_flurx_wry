use bevy::prelude::Query;
use bevy::window::Window;
use bevy_flurx::action::once;
use bevy_flurx::prelude::ActionSeed;

use bevy_flurx_ipc::prelude::WebviewEntity;

use crate::command;

#[command]
pub fn plugin_app_window_hide(
    WebviewEntity(entity): WebviewEntity
) -> ActionSeed {
    once::run(move |mut window: Query<&mut Window>| {
        if let Ok(mut win) = window.get_mut(entity) {
            win.visible = false;
        }
    })
}