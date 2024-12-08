use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::prelude::{ReflectDefault, Reflect};

/// Please see [`wry::WebViewBuilder::with_https_scheme`].
/// 
/// This setting only works on `Windows` and `Android`. 
#[repr(transparent)]
#[derive(Component, Copy, Clone, Reflect, Default, Eq, PartialEq)]
#[reflect(Component, Default)]
pub struct UseHttpsScheme(pub bool);
