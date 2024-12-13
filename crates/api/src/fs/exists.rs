use crate::fs::{error_if_not_accessible, join_path_if_need, BaseDirectory, FsScope};
use crate::macros::define_api_plugin;
use bevy_ecs::system::{In, Res};
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;
use serde::Deserialize;
use std::path::PathBuf;

define_api_plugin!(
    /// You'll be able to check if the path exists from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const existsPath: boolean = await window.__FLURX__.fs.exists("./dir");
    /// ```
    FsExistsPlugin,
    command: exists
);

#[derive(Deserialize, Default)]
struct Args {
    path: PathBuf,
    dir: Option<BaseDirectory>,
}

#[command(id = "FLURX|fs::exists", internal)]
fn exists(In(args): In<Args>) -> Action<Args, Result<bool, String>> {
    once::run(exists_system).with(args)
}

fn exists_system(
    In(args): In<Args>,
    scope: Option<Res<FsScope>>,
) -> Result<bool, String> {
    let path = join_path_if_need(&args.dir, args.path);
    error_if_not_accessible(&path, &scope)?;
    std::fs::exists(path).map_err(|e| e.to_string())
}


#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::exists::{exists_system, Args};
    use crate::fs::FsScope;
    use crate::tests::test_app;
    use bevy::utils::default;
    use bevy_app::{Startup, Update};
    use bevy_ecs::prelude::Commands;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Reactor, Then};

    #[test]
    fn test_exists() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let result: Result<_, _> = task.will(Update, once::run(exists_system).with(Args {
                    path: tmp_dir,
                    ..default()
                })).await;
                assert!(result.unwrap());
            }));
        });
        app.update();
    }

    #[test]
    fn test_not_exists() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let not_exists_dir = tmp_dir.join("not_exists");
                let result: Result<_, _> = task.will(Update, once::run(exists_system).with(Args {
                    path: not_exists_dir,
                    ..default()
                })).await;
                assert!(!result.unwrap());
            }));
        });
        app.update();
    }

    #[test]
    fn err_if_out_of_scope() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let result: Result<_, _> = task.will(Update, {
                    once::res::insert().with(FsScope::default())
                        .then(once::run(exists_system).with(Args {
                            path: tmp_dir,
                            ..default()
                        }))
                }).await;
                result.unwrap_err();
            }));
        });
        app.update();
    }
}