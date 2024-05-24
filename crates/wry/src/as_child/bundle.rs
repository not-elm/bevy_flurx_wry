use bevy::prelude::{Bundle, Component, Entity, Reflect, ReflectComponent};

pub use bounds::Bounds;
pub use toolbar::Toolbar;


pub(crate) mod resize;

mod bounds;
mod toolbar;

#[derive(Bundle)]
pub struct AsChild {
    pub parent: ParentWindow,

    pub bounds: Bounds,

    pub resizable: Resizable,
    
    pub toolbar: Toolbar
}

#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct ParentWindow(pub Entity);


#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Resizable(pub bool);

impl Default for Resizable {
    fn default() -> Self {
        Self(true)
    }
}

