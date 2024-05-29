use bevy::prelude::{Component, Reflect, ReflectDefault, ReflectComponent};


/// Please see [`wry::WebViewBuilder::with_incognito`] for details.
///
/// Default is `false`.
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, Component, Reflect)]
#[reflect(Component, Default)]
pub struct Incognito(pub bool);
