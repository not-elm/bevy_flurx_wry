use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};

/// Please see [`wry::WebViewBuilder::with_https_scheme`].
#[repr(transparent)]
#[derive(Component, Copy, Clone, Reflect, Default, Eq, PartialEq)]
#[reflect(Component, Default)]
pub struct HttpsScheme(pub bool);
