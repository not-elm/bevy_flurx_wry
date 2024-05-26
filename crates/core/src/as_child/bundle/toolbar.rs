use bevy::prelude::{Color, Component, Reflect, ReflectComponent, ReflectDefault};

/// Add a toolbar to the child Webview.
///
/// Currently, the toolbar is drawn using `bevy_ui`, which has some bugs.
/// Therefore, many breaking changes are anticipated in the future.
///
/// Note that you must spawn a [`WryWebViewBundle`](crate::prelude::WryWebViewBundle) and
/// [`AsChild`](crate::prelude::AsChildBundle) along with it.
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
///         },
///         Toolbar::default()
///     ));
/// }
/// ```
#[derive(Debug, Component, Reflect, PartialEq, Copy, Clone)]
#[reflect(Component, Default)]
pub struct Toolbar {
    /// Toolbar height(logical pixel).
    pub height: f32,

    /// Toolbar color.
    pub color: Color,
}

impl Default for Toolbar {
    fn default() -> Self {
        Self {
            height: 20.,
            color: Color::rgb_u8(0x3E, 0x3E, 0x3E),
        }
    }
}