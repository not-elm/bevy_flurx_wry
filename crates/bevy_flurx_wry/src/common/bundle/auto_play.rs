use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Please see [`wry::WebViewBuilder::with_autoplay`] for details.
/// 
/// Default is `false`.
#[repr(transparent)]
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct AutoPlay(pub bool);