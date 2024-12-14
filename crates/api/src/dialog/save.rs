use crate::dialog::DialogFilter;
use crate::fs::AllowPaths;
use crate::macros::define_api_plugin;
use bevy_app::Update;
use bevy_ecs::system::{In, ResMut};
use bevy_flurx::action::once;
use bevy_flurx::task::ReactiveTask;
use bevy_flurx_ipc::command;
use rfd::AsyncFileDialog;
use serde::Deserialize;
use std::path::PathBuf;

define_api_plugin!(
    /// You'll be able to open a file save dialog.
    ///
    ///  The selected path will be registered to [AllowPaths] until the application closed.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.dialog.save("question");
    /// ```
    DialogSavePlugin,
    command: save
);

#[derive(Default, Deserialize)]
struct Args {
    title: Option<String>,
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
    filters: Option<Vec<DialogFilter>>,
}

#[command(id = "FLURX|dialog::save", internal)]
async fn save(In(args): In<Args>, task: ReactiveTask) -> Option<PathBuf> {
    let path = select_save_path(args).await;
    task.will(Update, once::run(save_system).with(path.clone())).await;
    path
}

fn save_system(
    In(path): In<Option<PathBuf>>,
    allows: Option<ResMut<AllowPaths>>,
) {
    if let Some(mut allows) = allows {
        if let Some(path) = path {
            allows.add(path);
        }
    }
}

async fn select_save_path(args: Args) -> Option<PathBuf> {
    let mut dialog = AsyncFileDialog::new();
    dialog = dialog.set_can_create_directories(true);
    if let Some(title) = args.title {
        dialog = dialog.set_title(title);
    }
    if let Some(default_path) = args.default_path {
        dialog = dialog.set_directory(default_path);
    }
    if let Some(filters) = args.filters {
        for filter in filters {
            dialog = dialog.add_filter(filter.name, &filter.extensions);
        }
    }
    dialog.save_file().await.map(|h| h.path().to_path_buf())
}

