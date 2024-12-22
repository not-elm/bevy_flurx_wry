//! Provides a mechanism to control the basic behavior of Webview.

use bevy::prelude::{Component, Deref, DerefMut, Entity, Reflect, ReflectComponent};
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub mod plugin;
pub mod bundle;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::common::{
        bundle::csp::Csp,
        bundle::*,
        plugin::prelude::*,
        WebviewInitialized,
        WryWebViews,
    };
}

/// Marker component indicating that the webview has been initialized.
///
/// This is useful, for example, when setting up a custom API; As shown below.
///
/// ```no_run
/// use bevy_flurx_wry::prelude::*;
/// use bevy_flurx::prelude::*;
/// use bevy::prelude::*;
///
/// #[command]
/// fn custom_api() -> ActionSeed{
///     once::run(||{})
/// }
///
/// fn add_api(mut views: Query<&mut IpcHandlers, Added<WebviewInitialized>>){
///     for mut ipc_handler in views.iter_mut(){
///         ipc_handler.register(custom_api());
///     }
/// }
/// ```
#[derive(Component, Reflect, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct WebviewInitialized(pub ());


/// A hashmap that manages the initialized webview.
///
/// [`World`](bevy::prelude::World) holds this as [`NonSend`](bevy::prelude::NonSend).
#[repr(transparent)]
#[derive(Deref, DerefMut, Default)]
pub struct WryWebViews(pub(crate) HashMap<Entity, wry::WebView>);
