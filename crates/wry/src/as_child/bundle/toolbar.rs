use bevy::prelude::{Color, Component, Reflect, ReflectComponent, ReflectDefault};

#[derive(Debug, Component, Reflect, PartialEq, Copy, Clone)]
#[reflect(Component, Default)]
pub struct Toolbar {
    pub height: f32,

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