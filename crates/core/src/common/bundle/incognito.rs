use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::prelude::{ReflectDefault, Reflect};


/// Please see [`wry::WebViewBuilder::with_incognito`] for details.
///
/// Default is `false`.
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, Component, Reflect)]
#[reflect(Component, Default)]
pub struct Incognito(pub bool);
