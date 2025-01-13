use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Please see [`wry::WebViewBuilder::with_https_scheme`]().
///
/// This setting only works on `Windows` and `Android`.
#[repr(transparent)]
#[derive(Component, Copy, Clone, Reflect, Default, Eq, PartialEq, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct UseHttpsScheme(pub bool);
