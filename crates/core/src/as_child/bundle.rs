//! Declares the [`AsChildBundle`] and associated components.

use bevy::prelude::{Bundle, Component, Entity, Reflect, ReflectComponent};

pub use bounds::Bounds;

pub(crate) mod resize;

mod bounds;


/// Create the webview as a child of an existing [`Window`](bevy::prelude::Window).
///
/// Note that you must spawn a [`WryWebViewBundle`](crate::prelude::WryWebViewBundle) along with it.
///
/// ## Examples
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy::window::PrimaryWindow;
/// use bevy_flurx_wry_core::prelude::*;
///
/// fn spawn_webview_within_primary_window(
///     mut commands: Commands,
///     window: Query<Entity, With<PrimaryWindow>>
/// ){
///     commands.spawn((
///         WryWebViewBundle::default(),
///         AsChildBundle{
///             parent: ParentWindow(window.single()),
///             bounds: Bounds::default(),
///             resizable: Resizable::default()
///         }
///     ));
/// }
/// ```
#[derive(Bundle)]
pub struct AsChildBundle {
    /// The webview parent [`Window`](bevy::prelude::Window).
    pub parent: ParentWindow,

    ///  Represents the display area of a webview within the parent [`Window`](bevy::prelude::Window).
    pub bounds: Bounds,

    /// Whether to allow the webview to be resized.
    pub resizable: Resizable,
}


/// The webview parent window.
#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct ParentWindow(pub Entity);


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

