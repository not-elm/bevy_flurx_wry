//! Provides mechanism to access file systems from webview.

mod create_dir;
mod copy_file;
mod exists;
mod read_text_file;
mod remove_file;
mod rename_file;

use bevy_ecs::prelude::ReflectResource;
use bevy_ecs::system::{Res, Resource};
use bevy_reflect::prelude::ReflectDefault;
use bevy_reflect::Reflect;
use std::path::{Path, PathBuf};

pub use copy_file::FsCopyFilePlugin;
pub use create_dir::FsCreateDirPlugin;
pub use exists::FsExistsPlugin;
pub use read_text_file::FsReadTextFilePlugin;
pub use remove_file::FsRemoveFilePlugin;
pub use rename_file::FsRenameFilePlugin;

#[derive(Debug, Resource, Reflect, Default)]
#[reflect(Resource, Default)]
pub struct FsScope(Vec<PathBuf>);

impl FsScope {
    pub fn new<P: Into<PathBuf>>(allows: impl IntoIterator<Item=P>) -> Self {
        Self(allows.into_iter().map(|p| p.into()).collect())
    }

    fn check_accessible(&self, path: &str) -> bool {
        let path = Path::new(path);
        self.0.iter().any(|allow_path| {
            path.starts_with(allow_path)
        })
    }
}

fn error_if_not_accessible(
    path: &str,
    scope: &Option<Res<FsScope>>,
) -> Result<(), String> {
    if let Some(scope) = scope.as_ref() {
        if !scope.check_accessible(path) {
            return Err("Access to any of specified files isn't permitted by the application. ".to_string());
        }
    }
    Ok(())
}