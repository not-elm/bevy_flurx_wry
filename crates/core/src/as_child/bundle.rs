//! Declares the [`AsChildBundle`] and associated components.

use bevy_ecs::prelude::{Bundle, Component, Entity,ReflectComponent};
use bevy_reflect::Reflect;
pub use bounds::Bounds;
pub use grip_zone::GripZone;

pub(crate) mod resize;

mod bounds;
mod grip_zone;


/// Create the webview as a child of an existing [`Window`](bevy::prelude::Window).
///
/// Note that you must spawn a [`WryWebViewBundle`](crate::prelude::WryWebViewBundle) along with it.
///
/// ## Examples
///
/// ```no_run
/// use bevy_ecs::prelude::*;
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
///             resizable: Resizable::default(),
///             grip_zone: GripZone::default(),
///         }
///     ));
/// }
/// ```
#[derive(Bundle, Default)]
pub struct AsChildBundle {
    /// The webview parent [`Window`](bevy::prelude::Window).
    pub parent: ParentWindow,

    ///  Represents the display area of a webview within the parent [`Window`](bevy::prelude::Window).
    pub bounds: Bounds,

    /// Whether to allow the webview to be resized.
    pub resizable: Resizable,

    /// the height at which the webview can be gripped by a left-click.
    pub grip_zone: GripZone
}


/// The webview parent window.
/// 
/// ## Note
/// 
/// This component implements [`Default`] for [`AsChildBundle`],
/// but be sure to specify the correct a parent [`Window`](bevy::prelude::Window) entity for actual use.
#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct ParentWindow(pub Entity);

impl Default for ParentWindow{
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

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

