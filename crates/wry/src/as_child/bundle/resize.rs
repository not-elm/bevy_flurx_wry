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


#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use bevy::prelude::default;

    use crate::as_child::bundle::Bounds;
    use crate::prelude::resize::ResizeMode;

    #[test]
    fn resize_top() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(bounds.maybe_resizable(Vec2::new(6., 3.), None), Some(ResizeMode::Top));
    }

    #[test]
    fn resize_top_left() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(bounds.maybe_resizable(Vec2::new(4.5, 4.5), None), Some(ResizeMode::TopLeft));
    }

    #[test]
    fn resize_left() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(bounds.maybe_resizable(Vec2::new(4.5, 6.), None), Some(ResizeMode::Left));
    }

    #[test]
    fn resize_bottom_left() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(bounds.maybe_resizable(Vec2::new(4.5, 11.), None), Some(ResizeMode::BottomLeft));
    }

    #[test]
    fn resize_bottom() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(bounds.maybe_resizable(Vec2::new(7.5, 11.), None), Some(ResizeMode::Bottom));
    }

    #[test]
    fn resize_right() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(bounds.maybe_resizable(Vec2::new(13.5, 8.), None), Some(ResizeMode::Right));
    }

    #[test]
    fn resize_right_up() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(bounds.maybe_resizable(Vec2::new(11., 3.), None), Some(ResizeMode::TopRight));
    }

    #[test]
    fn resize_top_with_toolbar() {
        let bounds = Bounds {
            position: Vec2::new(5., 5.),
            size: Vec2::new(5., 5.),
            ..default()
        };
        assert_eq!(bounds.maybe_resizable(Vec2::new(6., 2.), Some(1.)), Some(ResizeMode::Top));
        assert_eq!(bounds.maybe_resizable(Vec2::new(6., 5.), Some(5.)), None);
    }
}