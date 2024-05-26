use bevy::prelude::{Component, CursorIcon, Reflect};

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
    pub(crate) fn cursor_icon(&self) -> CursorIcon {
        match self {
            Self::Top | Self::Bottom => CursorIcon::RowResize,
            Self::Left | Self::Right => CursorIcon::ColResize,
            Self::TopLeft => CursorIcon::NwResize,
            Self::BottomLeft => CursorIcon::SwResize,
            Self::BottomRight => CursorIcon::SeResize,
            Self::TopRight => CursorIcon::NeResize
        }
    }
}


