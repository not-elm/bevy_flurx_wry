//! Apply [Content Security Policy](https://developer.mozilla.org/ja/docs/Web/HTTP/Headers/Content-Security-Policy) for the webview. 

use bevy::prelude::{Component, Reflect, ReflectDefault, ReflectComponent};
use serde::{Deserialize, Serialize};

/// Represents the [Content Security Policy](https://developer.mozilla.org/ja/docs/Web/HTTP/Headers/Content-Security-Policy).
#[derive(Component, Default, Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct Csp(pub String);

impl From<&str> for Csp {
    fn from(value: &str) -> Self {
        Csp(value.to_string())
    }
}