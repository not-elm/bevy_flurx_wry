//! Provides the mechanism to spawn a webview as a child of an existing window using [`wry::WebViewBuilder::build_as_child`].

use bevy::prelude::{App, Plugin};
use bevy::prelude::{Component, ReflectComponent};
use bevy::prelude::{Reflect, Vec2};
use bevy_webview_core::bundle::embedding::{Bounds, EmbedWithin, Resizable};
use grip_zone::GripZonePlugin;
use resize::ResizePlugin;

mod resize;
mod grip_zone;

#[allow(missing_docs)]
pub mod prelude {
    pub use bevy_webview_core::bundle::embedding::*;
}

/// This is a webview that uses [`wry::WebViewBuilder::new_as_child`] to spawn a webview as a child of an existing window.
pub(crate) struct EmbeddingWebviewPlugin;

impl Plugin for EmbeddingWebviewPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<EmbedWithin>()
            .register_type::<Bounds>()
            .register_type::<Resizable>()
            .register_type::<CurrentMoving>()
            .add_plugins((
                ResizePlugin,
                GripZonePlugin,
            ));
    }
}
#[derive(Component, Reflect)]
#[reflect(Component)]
struct CurrentMoving(pub Vec2);


