use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};
use serde::{Deserialize, Serialize};


/// Represents the display destination of webview.
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub enum Uri {
    /// The name of local resources root dir.
    ///
    /// The root dir must be placed directly under the `assets/ui`,
    /// and `index.html`  must be placed directly under the root dir.
    ///
    /// For example, if target path is `assets/ui/example/index.html`, specify `example`.
    LocalRoot(String),

    /// The remote web uri
    Remote(String),
}

impl Default for Uri {
    fn default() -> Self {
        Uri::LocalRoot(".".to_string())
    }
}
