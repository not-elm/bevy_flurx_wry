use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};


/// Please see [`wry::WebViewBuilder::with_focused`] for details.
/// 
/// Default is `false`.
#[repr(transparent)]
#[derive(Component, Eq, PartialEq, Copy, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct InitializeFocused(pub bool);


