//! Provides apis to obtain special system paths.

use crate::macros::define_api_plugin;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_flurx::action::once;
use bevy_flurx::prelude::ActionSeed;
use bevy_flurx_ipc::command;
use std::path::PathBuf;
use bevy_ecs::system::Res;
use crate::error::ApiResult;
use crate::fs::FsScope;
use crate::prelude::error_if_not_accessible;

/// Allows you to use all path api plugins.
///
/// ## Plugins
///
/// - [PathConfigPlugin]
/// - [PathConfigLocalPlugin]
/// - [PathDataPlugin]
/// - [PathDataLocalPlugin]
/// - [PathAudioPlugin]
/// - [PathCachePlugin]
/// - [PathDesktopPlugin]
/// - [PathDocumentPlugin]
/// - [PathDownloadPlugin]
/// - [PathExecutablePlugin]
/// - [PathPublicPlugin]
/// - [PathRuntimePlugin]
/// - [PathTempPlugin]
/// - [PathTemplatePlugin]
/// - [PathVideoPlugin]
/// - [PathHomePlugin]
/// - [PathPicturePlugin]
/// - [PathFontPlugin]
pub struct AllPathPlugins;
impl PluginGroup for AllPathPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PathConfigPlugin)
            .add(PathConfigLocalPlugin)
            .add(PathDataPlugin)
            .add(PathDataLocalPlugin)
            .add(PathAudioPlugin)
            .add(PathCachePlugin)
            .add(PathDesktopPlugin)
            .add(PathDocumentPlugin)
            .add(PathDownloadPlugin)
            .add(PathExecutablePlugin)
            .add(PathPublicPlugin)
            .add(PathRuntimePlugin)
            .add(PathTempPlugin)
            .add(PathTemplatePlugin)
            .add(PathVideoPlugin)
            .add(PathHomePlugin)
            .add(PathPicturePlugin)
            .add(PathFontPlugin)
    }
}

define_api_plugin!(
    /// You'll be able to obtain user's config path from typescript(or js).
    /// 
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.cache();
    /// ```
    PathConfigPlugin,
    command: config
);

define_api_plugin!(
    /// You'll be able to obtain user's config local path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.configLocal();
    /// ```
    PathConfigLocalPlugin,
    command: config_local
);

define_api_plugin!(
    /// You'll be able to obtain user's data path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.data_local();
    /// ```
    PathDataLocalPlugin,
    command: data_local
);

define_api_plugin!(
    /// You'll be able to obtain user's data path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.data();
    /// ```
    PathDataPlugin,
    command: data
);

define_api_plugin!(
    /// You'll be able to obtain user's audio path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.audio();
    /// ```
    PathAudioPlugin,
    command: audio
);

define_api_plugin!(
    /// You'll be able to obtain user's cache path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.cache();
    /// ```
    PathCachePlugin,
    command: cache
);

define_api_plugin!(
    /// You'll be able to obtain user's desktop path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.desktop();
    /// ```
    PathDesktopPlugin,
    command: desktop
);

define_api_plugin!(
    /// You'll be able to obtain user's document path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.document();
    /// ```
    PathDocumentPlugin,
    command: document
);

define_api_plugin!(
    /// You'll be able to obtain user's download path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.download();
    /// ```
    PathDownloadPlugin,
    command: download
);

define_api_plugin!(
    /// You'll be able to obtain user's executable path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.executable();
    /// ```
    PathExecutablePlugin,
    command: executable
);

define_api_plugin!(
    /// You'll be able to obtain user's public path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.public();
    /// ```
    PathPublicPlugin,
    command: public
);

define_api_plugin!(
    /// You'll be able to obtain user's runtime path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.runtime();
    /// ```
    PathRuntimePlugin,
    command: runtime
);

define_api_plugin!(
    /// You'll be able to obtain user's temp path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string = await window.__FLURX__.path.temp();
    /// ```
    PathTempPlugin,
    command: temp
);

define_api_plugin!(
    /// You'll be able to obtain user's template path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.template();
    /// ```
    PathTemplatePlugin,
    command: template
);

define_api_plugin!(
    /// You'll be able to obtain user's video path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.video();
    /// ```
    PathVideoPlugin,
    command: video
);

define_api_plugin!(
    /// You'll be able to obtain user's home path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.home();
    /// ```
    PathHomePlugin,
    command: home
);

define_api_plugin!(
    /// You'll be able to obtain user's picture path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.picture();
    /// ```
    PathPicturePlugin,
    command: picture
);

define_api_plugin!(
    /// You'll be able to obtain user's font path from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const path: string | null = await window.__FLURX__.path.font();
    /// ```
    PathFontPlugin,
    command: font
);

#[command(id = "FLURX|path::config", internal)]
fn config() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::config_dir))
}

#[command(id = "FLURX|path::config_local", internal)]
fn config_local() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::config_local_dir))
}

#[command(id = "FLURX|path::data", internal)]
fn data() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::data_dir))
}

#[command(id = "FLURX|path::data_local", internal)]
fn data_local() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::data_local_dir))
}

#[command(id = "FLURX|path::audio", internal)]
fn audio() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::audio_dir))
}

#[command(id = "FLURX|path::cache", internal)]
fn cache() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::cache_dir))
}

#[command(id = "FLURX|path::desktop", internal)]
fn desktop() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::desktop_dir))
}

#[command(id = "FLURX|path::document", internal)]
fn document() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::document_dir))
}

#[command(id = "FLURX|path::download", internal)]
fn download() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::download_dir))
}

#[command(id = "FLURX|path::executable", internal)]
fn executable() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::executable_dir))
}

#[command(id = "FLURX|path::public", internal)]
fn public() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::public_dir))
}

#[command(id = "FLURX|path::runtime", internal)]
fn runtime() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::runtime_dir))
}

#[command(id = "FLURX|path::temp", internal)]
fn temp() -> ActionSeed<(), PathBuf> {
    once::run(std::env::temp_dir)
}

#[command(id = "FLURX|path::template", internal)]
fn template() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::template_dir))
}

#[command(id = "FLURX|path::video", internal)]
fn video() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::video_dir))
}

#[command(id = "FLURX|path::home", internal)]
fn home() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::home_dir))
}

#[command(id = "FLURX|path::picture", internal)]
fn picture() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::picture_dir))
}

#[command(id = "FLURX|path::font", internal)]
fn font() -> ActionSeed<(), ApiResult<Option<PathBuf>>> {
    once::run(obtain_path(dirs::font_dir))
}

fn obtain_path(
    f: fn() -> Option<PathBuf>
) -> impl Fn(Option<Res<FsScope>>) -> ApiResult<Option<PathBuf>>{
    move |scope: Option<Res<FsScope>>|{
        let Some(path) = f() else {
            return Ok(None);
        };
        error_if_not_accessible(&path, &scope)?;
        Ok(Some(path))
    }
}