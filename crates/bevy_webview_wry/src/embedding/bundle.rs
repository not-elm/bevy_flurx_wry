//! Declares the components which creates the webview as child.

use bevy::prelude::{Component, Entity, Reflect, ReflectComponent};
pub use bounds::Bounds;
pub use grip_zone::GripZone;

pub(crate) mod resize;

mod bounds;
mod grip_zone;

/// Holds the window entity to embed the webview in.
///
/// ## Note
///
///  Note that you must spawn a [`WebviewUri`](crate::prelude::WebviewUri) along with it.
///
/// ## Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy::window::PrimaryWindow;
/// use bevy_webview_wry::prelude::*;
///
/// fn spawn_webview_within_primary_window(
///     mut commands: Commands,
///     window: Query<Entity, With<PrimaryWindow>>
/// ){
///     commands.spawn((
///         WebviewUri::default(),
///         EmbedWithIn(window.single()),
///     ));
/// }
#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[require(Bounds, Resizable, GripZone)]
#[reflect(Component)]
pub struct EmbedWithIn(pub Entity);

/// Whether to allow the webview to be resized.
///
/// Default is `true`.
#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Resizable(pub bool);

impl Default for Resizable {
    fn default() -> Self {
        Self(true)
    }
}

