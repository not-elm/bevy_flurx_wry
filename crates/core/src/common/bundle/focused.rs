use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::prelude::{ReflectDefault, Reflect};


/// Please see [`wry::WebViewBuilder::with_focused`] for details.
/// 
/// Default is `false`.
#[repr(transparent)]
#[derive(Component, Eq, PartialEq, Copy, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct InitializeFocused(pub bool);


