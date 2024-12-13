//! Provides mechanism to access file systems from webview.

mod create_dir;
mod copy_file;
mod exists;
mod read_file;
mod remove_file;
mod rename_file;
mod write_file;
mod remove_dir;
mod read_dir;

use std::path::{Path, PathBuf};
use bevy_ecs::prelude::ReflectResource;
use bevy_ecs::system::{Res, Resource};
use bevy_reflect::prelude::ReflectDefault;
use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};
pub use copy_file::FsCopyFilePlugin;
pub use create_dir::FsCreateDirPlugin;
pub use exists::FsExistsPlugin;
pub use read_file::{FsReadTextFilePlugin, FsReadBinaryFilePlugin};
pub use read_dir::FsReadDirPlugin;
pub use remove_dir::FsRemoveDirPlugin;
pub use remove_file::FsRemoveFilePlugin;
pub use rename_file::FsRenameFilePlugin;
pub use write_file::{FsWriteBinaryFilePlugin, FsWriteTextFilePlugin};

#[derive(Debug, Resource, Reflect, Default, Clone)]
#[reflect(Resource, Default)]
pub struct FsScope(Vec<PathBuf>);

impl FsScope {
    pub fn new<P: Into<PathBuf>>(allows: impl IntoIterator<Item=P>) -> Self {
        Self(allows.into_iter().map(|p| p.into()).collect())
    }

    fn check_accessible(&self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        self.0.iter().any(|allow_path| {
            path.starts_with(allow_path)
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum BaseDirectory {
    ConfigLocal,
    Data,
    LocalData,
    Audio,
    Cache,
    Config,
    Desktop,
    Document,
    Download,
    Executable,
    Font,
    Home,
    Picture,
    Public,
    Runtime,
    Temp,
    Template,
    Video,
}

impl BaseDirectory {
    fn as_path(&self) -> Option<PathBuf> {
        match self {
            BaseDirectory::Data => dirs::data_dir(),
            BaseDirectory::LocalData => dirs::data_local_dir(),
            BaseDirectory::Audio => dirs::audio_dir(),
            BaseDirectory::Cache => dirs::cache_dir(),
            BaseDirectory::Config => dirs::config_dir(),
            BaseDirectory::ConfigLocal => dirs::config_local_dir(),
            BaseDirectory::Desktop => dirs::desktop_dir(),
            BaseDirectory::Document => dirs::document_dir(),
            BaseDirectory::Download => dirs::download_dir(),
            BaseDirectory::Executable => dirs::executable_dir(),
            BaseDirectory::Font => dirs::font_dir(),
            BaseDirectory::Home => dirs::home_dir(),
            BaseDirectory::Picture => dirs::picture_dir(),
            BaseDirectory::Public => dirs::public_dir(),
            BaseDirectory::Runtime => dirs::runtime_dir(),
            BaseDirectory::Temp => Some(std::env::temp_dir()),
            BaseDirectory::Template => dirs::template_dir(),
            BaseDirectory::Video => dirs::video_dir(),
        }
    }
}

fn join_path_if_need(base: &Option<BaseDirectory>, path: PathBuf) -> PathBuf {
    if let Some(base) = base.and_then(|base| base.as_path()) {
        base.join(path)
    } else {
        path
    }
}

fn error_if_not_accessible(
    path: impl AsRef<Path>,
    scope: &Option<Res<FsScope>>,
) -> Result<(), String> {
    if let Some(scope) = scope.as_ref() {
        if !scope.check_accessible(path) {
            return Err("Access to any of specified files isn't permitted by the application. ".to_string());
        }
    }
    Ok(())
}
