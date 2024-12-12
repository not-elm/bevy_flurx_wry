use crate::fs::{error_if_not_accessible, FsScope};
use crate::macros::define_api_plugin;
use bevy_ecs::system::{In, Res};
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;
use serde::Deserialize;
use std::io::Write;
use std::path::Path;

define_api_plugin!(
    /// You'll be able to rename a file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.fs.writeFile("./hoge.txt", "file contents", {
    ///     append: true,
    ///     recursive: true
    /// })
    /// ```
    FsWriteFilePlugin,
    command: write_file
);

#[derive(Deserialize)]
struct Args {
    path: String,
    content: String,
    append: Option<bool>,
    recursive: Option<bool>,
}

#[command(id = "FLURX|fs::write_file", internal)]
fn write_file(In(args): In<Args>) -> Action<Args, Result<(), String>> {
    once::run(write_file_system).with(args)
}

fn write_file_system(
    In(args): In<Args>,
    scope: Option<Res<FsScope>>,
) -> Result<(), String> {
    error_if_not_accessible(&args.path, &scope)?;
    let append = args.append.is_some_and(|append| append);
    let path : &Path= args.path.as_ref();
    if args.recursive.is_some_and(|recursive|recursive){
        if let Some(parent) = path.parent(){
            if !parent.exists(){
                std::fs::create_dir_all(parent).map_err(|e|e.to_string())?;
            }
        }
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(args.path)
        .map_err(|e| e.to_string())?;
    file.write_all(args.content.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}


#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::write_file::{write_file_system, Args};
    use crate::tests::test_app;
    use bevy_app::{Startup, Update};
    use bevy_ecs::prelude::Commands;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::Reactor;

    #[test]
    fn overwrite_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("write_file_hoge1.txt");
                let result: Result<_, _> = task.will(Update, once::run(write_file_system).with(Args {
                    append: None,
                    recursive: None,
                    path: hoge_path.to_str().unwrap().to_string(),
                    content: "hoge".to_string(),
                })).await;
                result.unwrap();
                assert_eq!(std::fs::read_to_string(hoge_path).unwrap(), "hoge");
            }));
        });
        app.update();
    }

    #[test]
    fn append_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("write_file_hoge2.txt");
                let _ = std::fs::remove_file(&hoge_path);
                let result: Result<_, _> = task.will(Update, once::run(write_file_system).with(Args {
                    append: Some(true),
                    recursive: None,
                    path: hoge_path.to_str().unwrap().to_string(),
                    content: "hoge".to_string(),
                })).await;
                result.unwrap();
                assert_eq!(std::fs::read_to_string(&hoge_path).unwrap(), "hoge");
                let result: Result<_, _> = task.will(Update, once::run(write_file_system).with(Args {
                    append: Some(true),
                    recursive: None,
                    path: hoge_path.to_str().unwrap().to_string(),
                    content: "hoge".to_string(),
                })).await;
                result.unwrap();
                assert_eq!(std::fs::read_to_string(&hoge_path).unwrap(), "hogehoge");
            }));
        });
        app.update();
    }

    #[test]
    fn err_if_not_specified_recursive() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("not_exists_dir").join("hoge1.txt");
                let result: Result<_, _> = task.will(Update, once::run(write_file_system).with(Args {
                    append: None,
                    recursive: None,
                    path: hoge_path.to_str().unwrap().to_string(),
                    content: "hoge".to_string(),
                })).await;
                result.unwrap_err();
            }));
        });
        app.update();
    }

    #[test]
    fn recursive_create_dir() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let hoge_path = tmp_dir.join("write_file").join("hoge2.txt");
                let result: Result<_, _> = task.will(Update, once::run(write_file_system).with(Args {
                    append: None,
                    recursive: Some(true),
                    path: hoge_path.to_str().unwrap().to_string(),
                    content: "hoge".to_string(),
                })).await;
                result.unwrap();
                assert_eq!(std::fs::read_to_string(hoge_path).unwrap(), "hoge");
            }));
        });
        app.update();
    }
}