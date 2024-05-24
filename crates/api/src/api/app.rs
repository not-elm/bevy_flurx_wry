use bevy::app::AppExit;
use bevy_flurx::action::{Action, once};
use bevy_flurx::prelude::ActionSeed;

pub fn get_name() -> ActionSeed<(), String> {
    once::run(|| {
        env!("CARGO_PKG_NAME").to_string()
    })
}

pub fn get_version() -> ActionSeed<(), String> {
    once::run(|| {
        env!("CARGO_PKG_VERSION").to_string()
    })
}

pub fn exit() -> Action<AppExit, ()> {
    once::event::app_exit()
}