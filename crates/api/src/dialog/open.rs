use crate::macros::define_api_plugin;
use bevy_ecs::system::In;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

define_api_plugin!(
    /// You'll be able to open a file/directory selection dialog.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.dialog.open("question");
    /// ```
    DialogOpenPlugin,
    command: open
);

#[derive(Default, Deserialize)]
struct Args {
    title: Option<String>,
    #[serde(rename = "defaultPath")]
    default_path: Option<String>,
    directory: Option<bool>,
    multiple: Option<bool>,
}

#[derive(Serialize)]
enum SelectedPaths {
    Single(Option<PathBuf>),
    Multiple(Option<Vec<PathBuf>>),
}

#[command(id = "FLURX|dialog::open", internal)]
fn open(In(args): In<Args>) -> Action<Args, SelectedPaths> {
    once::run(system).with(args)
}

fn system(In(args): In<Args>) -> SelectedPaths {
    let mut dialog = FileDialog::new();
    dialog = dialog.set_can_create_directories(true);
    if let Some(title) = args.title {
        dialog = dialog.set_title(title);
    }
    if let Some(default_path) = args.default_path {
        dialog = dialog.set_directory(default_path);
    }
    match (args.directory.unwrap_or(false), args.multiple.unwrap_or(false)) {
        (true, true) => SelectedPaths::Multiple(dialog.pick_folders()),
        (true, false) => SelectedPaths::Single(dialog.pick_folder()),
        (false, true) => SelectedPaths::Multiple(dialog.pick_files()),
        (false, false) => SelectedPaths::Single(dialog.pick_file()),
    }
}

