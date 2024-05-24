use bevy::prelude::{Color, Component, Reflect, ReflectComponent, ReflectDefault};

#[derive(Debug, Component, Reflect, PartialEq, Copy, Clone)]
#[reflect(Component, Default)]
pub struct Toolbar {
    pub height: f32,

    pub color: Color,
}

impl Toolbar {
    pub(crate) fn script(&self) -> String {
        let [r, g, b, a] = self.color.as_rgba_u8();
        include_str!("../../../scripts/toolbar.js")
            .replace(
                "<TOOLBAR_HEIGHT>",
                &format!("{}px", self.height),
            )
            .replace(
                "<TOOLBAR_COLOR>",
                &format!("#{r:X}{g:X}{b:X}{a:X}"),
            )
    }
}


impl Default for Toolbar {
    fn default() -> Self {
        Self {
            height: 20.,
            color: Color::NONE,
        }
    }
}