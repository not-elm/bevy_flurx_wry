//! Provides mechanism to control the application such as reading metadata, exiting the application, etc.

use crate::macros::api_plugin;
use bevy::prelude::AppExit;
use bevy_flurx::action::{once, Action};
use bevy_flurx::prelude::ActionSeed;
use bevy_flurx_ipc::command;

api_plugin!(
    /// You will be able to get the application name from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const appName: string = await window.__FLURX__.app.get_name()
    /// ```
    AppGetNameApiPlugin,
    command: get_name
);

api_plugin!(
    /// You will be able to get the application version from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const appVersion: string = await window.__FLURX__.app.get_version()
    /// ```
    AppGetVersionApiPlugin,
    command: get_version
);

api_plugin!(
    /// You will be able to exit application from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.app.exit()
    /// ```
    AppExitApiPlugin,
    command: exit
);

#[command(id = "FLURX|app::get_name", internal)]
fn get_name() -> ActionSeed<(), String> {
    get_name_action()
}

fn get_name_action() -> ActionSeed<(), String> {
    once::run(|| {
        env!("CARGO_PKG_NAME").to_string()
    })
}

#[command(id = "FLURX|app::get_version", internal)]
fn get_version() -> ActionSeed<(), String> {
    once::run(|| {
        env!("CARGO_PKG_VERSION").to_string()
    })
}

#[command(id = "FLURX|app::exit", internal)]
fn exit() -> Action<AppExit, ()> {
    once::event::app_exit_success()
}


#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Pipe, Reactor};
    use crate::app::{get_name_action, AppExitApiPlugin, AppGetNameApiPlugin, AppGetVersionApiPlugin};
    use crate::tests::{assert_api_registered, test_app};

    #[test]
    fn registered_get_name() {
        assert_api_registered(AppGetNameApiPlugin, "FLURX|app::get_name");
    }

    #[test]
    fn get_correct_app_name() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                task.will(Update, get_name_action()
                    .pipe(once::run(|In(name): In<String>| {
                        assert_eq!(name, "bevy_flurx_wry_api");
                    })),
                ).await;
            }));
        });
    }

    #[test]
    fn registered_get_version() {
        assert_api_registered(AppGetVersionApiPlugin, "FLURX|app::get_version");
    }

    #[test]
    fn registered_exit() {
        assert_api_registered(AppExitApiPlugin, "FLURX|app::exit");
    }
}