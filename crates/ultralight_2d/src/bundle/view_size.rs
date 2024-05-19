use bevy::math::UVec2;
use bevy::prelude::{Component, Deref, DerefMut, Reflect, ReflectComponent, ReflectDefault};

#[repr(transparent)]
#[derive(Component, Eq, PartialEq, Debug, Reflect, Hash, Deref, DerefMut)]
#[reflect(Component, Default)]
pub struct ViewSize(pub UVec2);

impl From<UVec2> for ViewSize {
    fn from(value: UVec2) -> Self {
        Self(value)
    }
}

impl Default for ViewSize{
    fn default() -> Self {
        Self(UVec2::splat(500))
    }
}