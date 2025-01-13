use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Please see [`wry::WebViewBuilder::with_clipboard`] for details.
///
/// Default is `true`.
#[repr(transparent)]
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct EnableClipboard(pub bool);

impl Default for EnableClipboard {
    fn default() -> Self {
        Self(true)
    }
}