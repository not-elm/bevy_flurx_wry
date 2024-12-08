#![allow(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

pub mod app;
pub mod log;

#[allow(missing_docs)]
pub mod prelude{
    pub use crate::{
        app::*,
        log::*,
    };
}

mod macros {
    macro_rules! define_api_plugin {
        (
            $(#[$meta:meta])*
            $plugin_name: ident,
            command: $api_command: ident
        ) => {
            $(#[$meta])*
            pub struct $plugin_name;
            impl bevy::prelude::Plugin for $plugin_name{
                fn build(&self, app: &mut bevy::prelude::App) {
                    use bevy::prelude::{Added, Query};
                    use bevy_flurx_ipc::prelude::IpcHandlers;
                    use bevy_flurx_wry_core::prelude::WebviewInitialized;
                    app.add_systems(bevy::prelude::PostUpdate, |mut views: Query<&mut IpcHandlers, Added<WebviewInitialized>>|{
                        for mut handlers in views.iter_mut(){ 
                            handlers.register($api_command());
                        } 
                    });
                }
            }
        };
    }

    pub(crate) use define_api_plugin;
}


#[cfg(test)]
mod tests {
    use bevy::app::{App, Plugin, Startup};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::{Commands, Query};
    use bevy_flurx_ipc::component::IpcHandlers;

    use bevy_flurx_ipc::FlurxIpcPlugin;
    use bevy_flurx_wry_core::common::WebviewInitialized;

    pub fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(FlurxIpcPlugin);
        app
    }
    
    pub fn assert_api_registered<P: Plugin>(plugin: P, ipc_id: &'static str) {
        let mut app = test_app();
        app.add_plugins(plugin);
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((
                IpcHandlers::default(),
                WebviewInitialized(()),
            ));
        });
        app.update();
        app.world_mut().run_system_once(move |handlers: Query<&IpcHandlers>| {
            assert!(handlers.single().get(ipc_id).is_some());
        }).expect("Failed to run system");
    }
}