//! Provides the mechanism to spawn a webview as a child of an existing window using [`wry::WebViewBuilder::new_as_child`].

use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_math::Vec2;
use bevy_reflect::prelude::Reflect;

pub mod bundle;
pub(super) mod plugin;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::as_child::bundle::*;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CurrentMoving(pub Vec2);
