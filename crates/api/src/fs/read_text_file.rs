use crate::fs::{error_if_not_accessible, FsScope};
use crate::macros::define_api_plugin;
use bevy_ecs::system::{In, Res};
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

define_api_plugin!(
    /// You'll be able to read a file as a UTF-8 encoded string from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const existsPath: boolean = await window.__FLURX__.fs.readTextFile("./dir");
    /// ```
    FsReadTextFilePlugin,
    command: read_text_file
);

#[command(id = "FLURX|fs::read_text_file", internal)]
fn read_text_file(In(args): In<String>) -> Action<String, Result<String, String>> {
    once::run(read_text_file_system).with(args)
}

fn read_text_file_system(
    In(path): In<String>,
    scope: Option<Res<FsScope>>,
) -> Result<String, String> {
    error_if_not_accessible(&path, &scope)?;
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}


#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::read_text_file::read_text_file_system;
    use crate::tests::test_app;
    use bevy_app::{Startup, Update};
    use bevy_ecs::prelude::Commands;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Reactor, Then};
    use crate::fs::FsScope;

    #[test]
    fn test_read_text_from_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("read_text_file_read_text_file.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let result: Result<String, _> = task.will(Update, once::run(read_text_file_system).with(hoge_path.to_str().unwrap().to_string())).await;
                assert_eq!(result.unwrap(), "hoge");
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
                let hoge_path = tmp_dir.join("read_text_file_read_text_file2.txt");
                std::fs::write(&hoge_path, "hoge").unwrap();
                let result: Result<String, _> = task.will(Update, {
                    once::res::insert().with(FsScope::default())
                        .then(once::run(read_text_file_system).with(hoge_path.to_str().unwrap().to_string()))
                }).await;
                result.unwrap_err();
            }));
        });
        app.update();
    }
}