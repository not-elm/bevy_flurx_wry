use bevy::prelude::Query;
use bevy::window::Window;
use bevy_flurx::action::once;
use bevy_flurx::prelude::ActionSeed;

use crate::command;

//TODO: window target
#[command]
pub fn plugin_window_show() -> ActionSeed {
    once::run(|mut window: Query<&mut Window>|{
        window.single_mut().visible = true;
    })
}