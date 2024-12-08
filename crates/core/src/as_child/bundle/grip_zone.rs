use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::prelude::{ReflectDefault, Reflect};

/// `GripZone` specifies the height at which the webview can be gripped
/// by a left-click.
/// 
/// You can move the webview by dragging it while holding it.
/// 
/// 
/// The unit of height is css pixels.
/// 
/// Default is 20(px).
#[repr(transparent)]
#[derive(Debug, Reflect, Copy, Clone, Eq, PartialEq, Hash, Component)]
#[reflect(Component, Default)]
pub struct GripZone(pub u32);

impl Default for GripZone{
    fn default() -> Self {
        Self(20)
    }
}


