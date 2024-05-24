use bevy::prelude::{Component, Reflect, ReflectDefault, ReflectComponent};


/// Please see [`wry::WebViewBuilder::with_incognito`].
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, Component, Reflect)]
#[reflect(Component, Default)]
pub struct Incognito(pub bool);
