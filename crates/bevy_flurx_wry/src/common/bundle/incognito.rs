use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Please see [`wry::WebViewBuilder::with_incognito`] for details.
///
/// Default is `false`.
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
pub struct Incognito(pub bool);
