use bevy::app::{App, };
use bevy::math::Vec2;
use bevy::prelude::{Bundle, Component, Entity, Plugin, Reflect, ReflectComponent, ReflectDefault};

#[derive(Bundle)]
pub struct AsChild {
    pub parent: ParentWindow,

    pub bounds: Bounds,
}


#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct ParentWindow(pub Entity);

#[derive(Component, PartialEq, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Bounds {
    pub size: Vec2,

    pub position: Vec2,
}

impl Bounds {
    pub(crate) fn as_wry_rect(&self) -> wry::Rect {
        wry::Rect {
            position: wry::dpi::LogicalPosition::new(self.position.x, self.position.y).into(),
            size: wry::dpi::LogicalSize::new(self.size.x, self.size.y).into(),
        }
    }
}


pub struct AsChildPlugin;

impl Plugin for AsChildPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ParentWindow>()
            .register_type::<Bounds>();
    }
}

