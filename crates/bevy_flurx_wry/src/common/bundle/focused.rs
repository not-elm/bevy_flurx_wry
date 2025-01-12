use bevy::prelude::{Component, ReflectComponent};
use bevy::prelude::{ReflectDefault, Reflect};


/// Please see [`wry::WebViewBuilder::with_focused`] for details.
/// 
/// Default is `false`.
#[repr(transparent)]
#[derive(Component, Eq, PartialEq, Copy, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct InitializeFocused(pub bool);


