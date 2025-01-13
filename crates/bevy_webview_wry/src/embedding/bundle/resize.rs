use bevy::prelude::{Component, Reflect};
use bevy::window::SystemCursorIcon;


#[derive(Component, Reflect, Eq, PartialEq, Debug, Copy, Clone)]
pub(crate) enum ResizeMode {
    Left,
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
}

impl ResizeMode {
    pub(crate) fn cursor_icon(&self) -> SystemCursorIcon {
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
