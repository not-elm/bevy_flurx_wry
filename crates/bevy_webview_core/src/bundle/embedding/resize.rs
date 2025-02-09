use bevy::prelude::{Component, Reflect};
use bevy::window::SystemCursorIcon;


/// Represents the resize direction of the embedded webview.
/// To enable resizing, [`Resizable`](crate::bundle::embedding::Resizable) must be true.
#[derive(Component, Reflect, Eq, PartialEq, Debug, Copy, Clone)]
pub enum ResizeMode {
    /// Resize to the left.
    Left,
    /// Resize to the top-left.
    TopLeft,
    /// Resize to the top.
    Top,
    /// Resize to the top-right.
    TopRight,
    /// Resize to the right.
    Right,
    /// Resize to the bottom-right.
    BottomRight,
    /// Resize to the bottom.
    Bottom,
    /// Resize to the bottom-left.
    BottomLeft,
}

impl ResizeMode {
    /// Returns the cursor icon for the resize direction.
    pub fn cursor_icon(&self) -> SystemCursorIcon {
        match self {
            Self::Top | Self::Bottom => SystemCursorIcon::RowResize,
            Self::Left | Self::Right => SystemCursorIcon::ColResize,
            Self::TopLeft => SystemCursorIcon::NwResize,
            Self::BottomLeft => SystemCursorIcon::SwResize,
            Self::BottomRight => SystemCursorIcon::SeResize,
            Self::TopRight => SystemCursorIcon::NeResize,
        }
    }
}
