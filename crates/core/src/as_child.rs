use bevy::math::Vec2;
use bevy::prelude::{Component, Reflect, ReflectComponent};

pub mod bundle;
pub mod plugin;

pub mod prelude {
    pub use crate::as_child::{
        bundle::*,
        plugin::AsChildPlugin
    };
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct CurrentMoving(pub Vec2);

