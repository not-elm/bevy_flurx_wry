use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::prelude::{ReflectDefault, Reflect};
use serde::{Deserialize, Serialize};

/// Please see [`wry::WebViewBuilder::with_clipboard`] for details.
/// 
/// Default is `true`.
#[repr(transparent)]
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct EnableClipboard(pub bool);

impl Default for EnableClipboard{
    fn default() -> Self {
        Self(true)
    }
}