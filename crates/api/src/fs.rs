//! Provides mechanism to access file systems from webview.

use crate::macros::define_api_plugin;
use bevy_ecs::prelude::ReflectResource;
use bevy_ecs::system::{In, Res, Resource};
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;
use bevy_flurx_ipc::command;
use bevy_reflect::prelude::ReflectDefault;
use bevy_reflect::Reflect;
use std::path::PathBuf;

#[derive(Debug, Resource, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct FsScope(Vec<PathBuf>);

impl FsScope {
    pub fn new<P: Into<PathBuf>>(allows: impl IntoIterator<Item=P>) -> Self {
        Self(allows.into_iter().map(|p| p.into()).collect())
    }

    fn check_accessible(&self, path: &str) -> bool {
        self.0.iter().any(|allow_path| {
            allow_path.starts_with(path)
        })
    }
}

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
    if let Some(scope) = scope {
        if !scope.check_accessible(&source) || !scope.check_accessible(&destination) {
            return Err("Access to any of specified files isn't permitted by the application.".to_string());
        }
    }
    std::fs::copy(source, destination).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::fs::{copy_file_system, FsScope};
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
                let src = tmp_dir.join("source.txt");
                let dest = tmp_dir.join("dest.txt");
                std::fs::write(&src, "hello").unwrap();
                let result: Result<_, _> = task.will(Update, once::run(copy_file_system).with((src.to_str().unwrap().to_string(), dest.to_str().unwrap().to_string()))).await;
                assert!(result.is_ok());
            }));
        });
    }

    #[test]
    fn copy_file_not_permitted_access() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                // Access to any files is not permitted.
                task.will(Update, once::res::insert().with(FsScope::default())).await;
                let tmp_dir = std::env::temp_dir();
                let src = tmp_dir.join("source.txt");
                let dest = tmp_dir.join("dest.txt");
                std::fs::write(&src, "hello").unwrap();
                let result: Result<_, _> = task.will(Update, once::run(copy_file_system).with((src.to_str().unwrap().to_string(), dest.to_str().unwrap().to_string()))).await;
                assert!(result.is_err());
            }));
        });
    }
    
    #[test]
    fn copy_file_permitted_access() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                // Access to any files is not permitted.
                task.will(Update, once::res::insert().with(FsScope::new([
                    "source.txt",
                    "dest.txt",
                ]))).await;
                let tmp_dir = std::env::temp_dir();
                let src = tmp_dir.join("source.txt");
                let dest = tmp_dir.join("dest.txt");
                std::fs::write(&src, "hello").unwrap();
                let result: Result<_, _> = task.will(Update, once::run(copy_file_system).with((src.to_str().unwrap().to_string(), dest.to_str().unwrap().to_string()))).await;
                assert!(result.is_ok());
            }));
        });
    }
}