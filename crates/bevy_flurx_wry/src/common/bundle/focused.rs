use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Please see [`wry::WebViewBuilder::with_focused`] for details.
///
/// Default is `false`.
#[repr(transparent)]
#[derive(Component, Eq, PartialEq, Copy, Clone, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct InitializeFocused(pub bool);


