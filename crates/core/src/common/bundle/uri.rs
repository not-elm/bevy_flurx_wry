use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Represents the display destination of webview.
///
/// Default is `Uri::Local("flurx://localhost/".to_string())`.
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub enum Uri {
    /// Pass a local url in the form of a custom protocol.
    ///
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_flurx_wry_core::prelude::*;
    /// use std::path::PathBuf;
    /// use bevy::window::PrimaryWindow;
    ///
    /// App::new()
    ///     .add_plugins((
    ///         DefaultPlugins,
    ///         FlurxWryPlugin{
    ///             local_root: PathBuf::from("ui")
    ///         }
    ///     ))
    ///     .run();
    ///
    /// fn spawn_webview(mut commands: Commands, window: Query<Entity, With<PrimaryWindow>>){
    ///     commands.entity(window.single()).insert(WryWebViewBundle{
    ///         // show assets/ui/example.html
    ///         uri: Uri::Local("flurx://localhost/example.html".to_string()),
    ///         ..default()
    ///     });
    /// }
    /// ```
    Local(String),

    /// The remote web uri
    Remote(String),
}

impl Default for Uri {
    fn default() -> Self {
        Uri::Local("flurx://localhost/".to_string())
    }
}
