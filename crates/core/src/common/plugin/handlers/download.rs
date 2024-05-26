use std::path::PathBuf;

use bevy::app::App;
use bevy::prelude::{Entity, Event, Plugin, Reflect};

use crate::common::plugin::handlers::RegisterWryEvent;

#[derive(Clone, Debug, Event, Reflect)]
pub struct DownloadStarted {
    pub webview_entity: Entity,

    /// The url being downloaded from.
    pub source_url: String,

    /// Download destination.
    pub dest: PathBuf,
}


/// Fired when the download completes, whether it was successful or not.
///
/// Please see details for [`wry::WebViewBuilder::with_download_completed_handler`].
#[derive(Clone, Debug, Event, Reflect)]
pub struct DownloadCompleted {
    pub webview_entity: Entity,

    /// The url original download request
    pub source_url: String,

    /// Potentially representing the filesystem path the file was downloaded to.
    pub dest: Option<PathBuf>,

    /// Whether download succeed or not.
    pub succeed: bool,
}


pub(super) struct DownloadPlugin;

impl Plugin for DownloadPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_wry_event::<DownloadStarted>()
            .register_wry_event::<DownloadCompleted>();
    }
}


