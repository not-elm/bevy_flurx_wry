use bevy::app::Startup;
use bevy::DefaultPlugins;
use bevy::prelude::{App, Camera2dBundle, Commands, ResMut, Resource};
use bevy::utils::default;
use bevy_flurx::action::once;
use bevy_flurx::prelude::ActionSeed;

use bevy_flurx_ipc::{command, ipc_handlers};
use ultralight_2d::bundle::{Load, Ul2DWebViewBundle};
use ultralight_2d::plugin::FlurxUlPlugin;


#[derive(Resource, Default)]
struct Count(usize);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlurxUlPlugin::default(),
        ))
        .init_resource::<Count>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Ul2DWebViewBundle {
        load: Load::Uri("file:///count_up.html".to_string()),
        ipc_handlers: ipc_handlers![
            count_up
        ],
        ..default()
    });
}

#[command]
fn count_up() -> ActionSeed<(), usize> {
    once::run(increment)
}

fn increment(mut count: ResMut<Count>) -> usize {
    count.0 += 1;
    count.0
}