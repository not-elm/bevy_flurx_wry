use crate::fs::{error_if_not_accessible, FsScope};
use crate::macros::define_api_plugin;
use bevy_ecs::system::{In, Res};
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

define_api_plugin!(
    /// You'll be able to rename a file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.renameFile("./old.txt", "./new.txt");
    /// ```
    FsRenameFilePlugin,
    command: rename_file
);

type Args = (String, String);

#[command(id = "FLURX|fs::rename_file", internal)]
fn rename_file(In(args): In<Args>) -> Action<Args, Result<(), String>> {
    once::run(rename_file_system).with(args)
}

fn rename_file_system(
    In(args): In<Args>,
    scope: Option<Res<FsScope>>,
) -> Result<(), String> {
    error_if_not_accessible(&args.0, &scope)?;
    error_if_not_accessible(&args.1, &scope)?;
    std::fs::rename(args.0, args.1).map_err(|e| e.to_string())
}


#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::rename_file::rename_file_system;
    use crate::tests::test_app;
    use bevy_app::{Startup, Update};
    use bevy_ecs::prelude::Commands;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Reactor, Then};
    use crate::fs::FsScope;

    #[test]
    fn test_remove_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("rename_file_old1.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let old_path = hoge_path.to_str().unwrap().to_string();
                let new_path = tmp_dir.join("rename_file_new1.txt");
                let result: Result<_, _> = task.will(Update, once::run(rename_file_system).with((old_path.clone(), new_path.to_str().unwrap().to_string()))).await;
                result.unwrap();
                assert!(!std::fs::exists(old_path).unwrap());
                assert!(std::fs::exists(new_path).unwrap());
            }));
        });
        app.update();
    }

    #[test]
    fn error_out_of_scope() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("rename_file_old2.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let old_path = hoge_path.to_str().unwrap().to_string();
                let new_path = tmp_dir.join("rename_file_new2.txt");
                let result: Result<_, _> = task.will(Update, {
                    once::res::insert().with(FsScope::default())
                        .then(once::run(rename_file_system).with((old_path.clone(), new_path.to_str().unwrap().to_string())))
                }).await;
                result.unwrap_err();
                assert!(std::fs::exists(old_path).unwrap());
                assert!(!std::fs::exists(new_path).unwrap());
            }));
        });
        app.update();
    }
}