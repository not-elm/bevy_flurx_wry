//! Provides the mechanism to spawn a webview as a child of an existing window using [`wry::WebViewBuilder::build_as_child`].

use bevy::prelude::{Component, ReflectComponent};
use bevy::prelude::{Reflect, Vec2};

pub mod bundle;
pub(super) mod plugin;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::embedding::bundle::*;
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CurrentMoving(pub Vec2);
