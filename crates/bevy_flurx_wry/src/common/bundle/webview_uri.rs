use crate::common::bundle::*;
use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use bevy_flurx_ipc::prelude::IpcHandlers;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Represents the display destination of webview.
///
/// If you want to load a local resource, use custom protocol: `flurx://localhost/<ROOT>/<uri>`.
///
/// `<ROOT>` is specified by [`FlurxWryPlugin::local_root`](crate::prelude::FlurxWryPlugin).
///
///
/// Default is `flurx://localhost/`.
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_flurx_wry::prelude::*;
/// use std::path::PathBuf;
/// use bevy::window::PrimaryWindow;
///
/// App::new()
///     .add_plugins((
///         DefaultPlugins,
///         FlurxWryPlugin{
///             // local root will be flurx://localhost/ui. 
///             local_root: PathBuf::from("ui")
///         }
///     ))
///     .run();
///
/// fn spawn_webview(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>){
///     commands
///         .entity(window.single())
///         // The actual URL is flurx://localhost/ui/example.html.
///         // show assets/ui/example.html
///         .insert(WebviewUri::relative_local("example.html"));
/// }
/// ```
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[require(
    EnableClipboard,
    AutoPlay,
    BrowserAcceleratorKeys,
    UseDevtools,
    IsOpenDevtools,
    WebviewVisible,
    Background,
    UserAgent,
    Theme,
    InitializeFocused,
    InitializationScripts,
    Incognito,
    HotkeysZoom,
    UseHttpsScheme,
    IpcHandlers,
    OnDownload,
    OnDragDrop,
    OnNavigation,
    OnNewWindowRequest,
    EventEmitter,
)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct WebviewUri(pub String);

impl WebviewUri {
    /// Returns the new [`WebviewUri`].
    pub fn new(uri: impl Into<String>) -> Self {
        Self(uri.into())
    }

    /// Returns the webview uri to load a local resource.
    ///
    /// The url will be in the form of a custom protocol: `flurx://localhost/<ROOT>/<uri>`.
    ///
    /// `<ROOT>` is specified in the [`FlurxWryPlugin`](crate::prelude::FlurxWryPlugin).
    pub fn relative_local(uri: impl AsRef<Path>) -> Self {
        let path = PathBuf::from("flurx://localhost/").join(uri.as_ref());
        Self(path.to_string_lossy().to_string())
    }
}

impl Default for WebviewUri {
    fn default() -> Self {
        WebviewUri::relative_local("")
    }
}
