use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};

#[derive(Component, Default, Reflect, PartialEq, Copy, Clone)]
#[reflect(Component, Default)]
pub enum Toolbar {
    #[default]
    None,

    Px(f32),
}

impl Toolbar {
    pub(crate) fn script(&self) -> Option<String> {
        let toolbar_height = self.as_height()?;
        Some(include_str!("../../../scripts/toolbar.js").replace(
            "<TOOLBAR_HEIGHT>",
            &toolbar_height,
        ))
    }

    fn as_height(&self) -> Option<String> {
        match self {
            Self::None => None,
            Self::Px(px) => Some(format!("{px}px")),
        }
    }
}
