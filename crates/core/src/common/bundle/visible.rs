use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::prelude::ReflectDefault;
use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};


/// Please see [`wry::WebViewBuilder::with_visible`] for details.
///
/// Default is `true`.
#[repr(transparent)]
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct WebviewVisible(pub bool);

impl Default for WebviewVisible {
    fn default() -> Self {
        Self(true)
    }
}


