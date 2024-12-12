use crate::fs::{error_if_not_accessible, FsScope};
use crate::macros::define_api_plugin;
use bevy_ecs::system::{In, Res};
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

define_api_plugin!(
    /// You'll be able to remove file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.removeFile("./hoge.txt");
    /// ```
    FsRemoveFilePlugin,
    command: remove_file
);

#[command(id = "FLURX|fs::remove_file", internal)]
fn remove_file(In(args): In<String>) -> Action<String, Result<(), String>> {
    once::run(remove_file_system).with(args)
}

fn remove_file_system(
    In(path): In<String>,
    scope: Option<Res<FsScope>>,
) -> Result<(), String> {
    error_if_not_accessible(&path, &scope)?;
    std::fs::remove_file(path).map_err(|e| e.to_string())
}


#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::remove_file::remove_file_system;
    use crate::fs::FsScope;
    use crate::tests::test_app;
    use bevy_app::{Startup, Update};
    use bevy_ecs::prelude::Commands;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Reactor, Then};

    #[test]
    fn test_remove_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("remove_file_hoge1.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let result: Result<_, _> = task.will(Update, once::run(remove_file_system).with(hoge_path.to_str().unwrap().to_string())).await;
                result.unwrap();
                assert!(!std::fs::exists(hoge_path).unwrap());
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
                let hoge_path = tmp_dir.join("remove_file_hoge2.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let result: Result<_, _> = task.will(Update, {
                    once::res::insert().with(FsScope::default())
                        .then(once::run(remove_file_system).with(hoge_path.to_str().unwrap().to_string()))
                }).await;
                result.unwrap_err();
                assert!(std::fs::exists(hoge_path).unwrap());
            }));
        });
        app.update();
    }
}