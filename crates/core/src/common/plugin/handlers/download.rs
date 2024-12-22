//! Controls download events.

use std::path::PathBuf;
use bevy::prelude::{App, Entity, Event, Plugin, Reflect};
use crate::common::plugin::handlers::RegisterWryEvent;
use crate::prelude::PassedUrl;


/// Fired when a download is requested and allowed in the [`OnDownload`](crate::prelude::OnDownload) callback.
///
/// Please see [`wry::WebViewBuilder::with_download_started_handler`] for detail.
#[derive(Clone, Debug, Event, Reflect)]
pub struct DownloadStarted {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// The url being downloaded from.
    pub source_url: PassedUrl,

    /// Download destination.
    pub dest: PathBuf,
}

/// Fired when the download completes, whether it was successful or not.
///
/// Please see [`wry::WebViewBuilder::with_download_completed_handler`] for detail.
#[derive(Clone, Debug, Event, Reflect)]
pub struct DownloadCompleted {
    /// The entity associated with the webview from which this event was fired.
    pub webview_entity: Entity,

    /// The url original download request
    pub source_url: PassedUrl,

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


