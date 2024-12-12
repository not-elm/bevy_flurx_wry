use crate::fs::{error_if_not_accessible, FsScope};
use crate::macros::define_api_plugin;
use bevy_ecs::system::{In, Res};
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;
use serde::Deserialize;

define_api_plugin!(
    /// You'll be able to create dirs from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.createDir("./dir");
    /// ```
    FsCreateDirPlugin,
    command: create_dir
);

#[derive(Deserialize)]
struct CreateDirArgs {
    dir_path: String,
    recursive: Option<bool>,
}

#[command(id = "FLURX|fs::create_dir", internal)]
fn create_dir(In(args): In<CreateDirArgs>) -> Action<CreateDirArgs, Result<(), String>> {
    once::run(create_dir_system).with(args)
}

fn create_dir_system(
    In(args): In<CreateDirArgs>,
    scope: Option<Res<FsScope>>,
) -> Result<(), String> {
    error_if_not_accessible(&args.dir_path, &scope)?;
    if std::fs::exists(&args.dir_path).is_ok_and(|exists| exists) {
        return Ok(());
    }
    if args.recursive.is_some_and(|recursive| recursive) {
        std::fs::create_dir_all(args.dir_path).map_err(|e| e.to_string())
    } else {
        std::fs::create_dir(args.dir_path).map_err(|e| e.to_string())
    }
}


#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::create_dir::{create_dir_system, CreateDirArgs};
    use crate::fs::FsScope;
    use crate::tests::test_app;
    use bevy_app::{Startup, Update};
    use bevy_ecs::prelude::Commands;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::Reactor;

    #[test]
    fn test_create_dir() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let result: Result<_, _> = task.will(Update, once::run(create_dir_system).with(CreateDirArgs {
                    dir_path: tmp_dir.join("dir1").to_str().unwrap().to_string(),
                    recursive: None,
                })).await;
                assert!(result.is_ok());
            }));
        });
        app.update();
    }

    #[test]
    fn create_fail_if_not_specified_recursive() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let result: Result<_, _> = task.will(Update, once::run(create_dir_system).with(CreateDirArgs {
                    dir_path: tmp_dir.join("not_recursive").join("dir").to_str().unwrap().to_string(),
                    recursive: None,
                })).await;
                assert!(result.is_err());
            }));
        });
        app.update();
    }

    #[test]
    fn ok_if_specified_recursive() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let result: Result<_, _> = task.will(Update, once::run(create_dir_system).with(CreateDirArgs {
                    dir_path: tmp_dir.join("recursive").join("dir").to_str().unwrap().to_string(),
                    recursive: Some(true),
                })).await;
                assert!(result.is_ok());
            }));
        });
        app.update();
    }

    #[test]
    fn out_of_fs_scope() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                task.will(Update, once::res::insert().with(FsScope::default())).await;
                let tmp_dir = std::env::temp_dir();
                let result: Result<_, _> = task.will(Update, once::run(create_dir_system).with(CreateDirArgs {
                    dir_path: tmp_dir.join("dir").to_str().unwrap().to_string(),
                    recursive: Some(true),
                })).await;
                assert!(result.is_err());
            }));
        });
        app.update();
        app.update();
    }
}