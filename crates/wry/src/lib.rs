#![allow(clippy::type_complexity)]


use std::path::PathBuf;

use bevy::app::{App, Plugin};
use bevy::prelude::{Reflect, Resource};

pub use bevy_flurx_ipc::{command, ipc_handlers};

use crate::as_child::plugin::AsChildPlugin;
use crate::core::plugin::FlurxWryCorePlugin;

pub mod as_child;
pub mod core;


#[allow(missing_docs)]
pub mod prelude {
    pub use crate::{
        as_child::prelude::*,
        command,
        core::prelude::*,
        FlurxWryPlugin,
        ipc_handlers,
    };
}


pub struct FlurxWryPlugin {
    pub content_root: PathBuf,
}

#[repr(transparent)]
#[derive(Resource, Debug, Reflect, Clone)]
pub(crate) struct WryLocalRoot(pub PathBuf);

impl Plugin for FlurxWryPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<WryLocalRoot>()
            .insert_resource(WryLocalRoot(self.content_root.clone()))
            .add_plugins((
                FlurxWryCorePlugin,
                AsChildPlugin
            ));
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use bevy::app::{App, PluginGroup};
    use bevy::DefaultPlugins;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Entity, NonSend, Query, With};
    use bevy::utils::default;
    use bevy::window::{Window, WindowPlugin};
    use bevy_test_helper::BevyTestHelperPlugin;

    use crate::core::WryWebViews;
    use crate::FlurxWryPlugin;

    pub fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                ..default()
            }),
            FlurxWryPlugin,
            BevyTestHelperPlugin
        ));
        app
    }

    pub trait EvaluateScript {
        fn evaluate_script(&mut self, script: impl Into<String>);
    }

    impl EvaluateScript for App {
        fn evaluate_script(&mut self, script: impl Into<String>) {
            let script = script.into();
            self.world.run_system_once(move |web_views: NonSend<WryWebViews>,
                                             views: Query<Entity, With<Window>>| {
                web_views.get(&views.single()).unwrap().evaluate_script(&script).unwrap();
            });
        }
    }
}