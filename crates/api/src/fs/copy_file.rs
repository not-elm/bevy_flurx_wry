use crate::fs::{error_if_not_accessible, FsScope};
use crate::macros::define_api_plugin;
use bevy_ecs::change_detection::Res;
use bevy_ecs::prelude::In;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

define_api_plugin!(
    /// You'll be able to copy file from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const source = "./hello.txt"
    /// const destination = "./destination/hello.txt"
    /// await window.__FLURX__.fs.copyFile(source, destination)
    /// ```
    FsCopyFilePlugin,
    command: copy_file
);

#[command(id = "FLURX|fs::copy_file", internal)]
fn copy_file(In(args): In<(String, String)>) -> Action<(String, String), Result<(), String>> {
    once::run(copy_file_system).with(args)
}

fn copy_file_system(
    In((source, destination)): In<(String, String)>,
    scope: Option<Res<FsScope>>,
) -> Result<(), String> {
    error_if_not_accessible(&source, &scope)?;
    error_if_not_accessible(&destination, &scope)?;
    std::fs::copy(source, destination).map_err(|e| e.to_string())?;
    Ok(())
}


#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::copy_file::copy_file_system;
    use crate::fs::FsScope;
    use crate::tests::test_app;
    use bevy_app::{Startup, Update};
    use bevy_ecs::prelude::Commands;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::Reactor;

    #[test]
    fn test_copy_file() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                let src = tmp_dir.join("source0.txt");
                let dest = tmp_dir.join("dest0.txt");
                std::fs::write(&src, "hello").unwrap();
                let result: Result<_, _> = task.will(Update, once::run(copy_file_system).with((src.to_str().unwrap().to_string(), dest.to_str().unwrap().to_string()))).await;
                result.unwrap();
            }));
        });
        app.update();
    }

    #[test]
    fn copy_file_not_permitted_access() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                // Access to any files is not permitted.
                task.will(Update, once::res::insert().with(FsScope::default())).await;
                let tmp_dir = std::env::temp_dir();
                let src = tmp_dir.join("source1.txt");
                let dest = tmp_dir.join("dest1.txt");
                std::fs::write(&src, "hello").unwrap();
                let result: Result<_, _> = task.will(Update, once::run(copy_file_system).with((src.to_str().unwrap().to_string(), dest.to_str().unwrap().to_string()))).await;
                result.unwrap_err();
            }));
        });
        app.update();
        app.update();
    }

    #[test]
    fn copy_file_permitted_access() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let tmp_dir = std::env::temp_dir();
                // Access to any files is not permitted.
                task.will(Update, once::res::insert().with(FsScope::new([
                    tmp_dir.clone(),
                ]))).await;
                let src = tmp_dir.join("source2.txt");
                let dest = tmp_dir.join("dest2.txt");
                std::fs::write(&src, "hello").unwrap();
                let result: Result<_, _> = task.will(Update, once::run(copy_file_system).with((src.to_str().unwrap().to_string(), dest.to_str().unwrap().to_string()))).await;
                result.unwrap();
            }));
        });
        app.update();
        app.update();
    }
}