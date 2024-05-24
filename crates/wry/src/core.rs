use bevy::prelude::{Component, Deref, DerefMut, Entity, Reflect, ReflectComponent};
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub mod plugin;
pub mod bundle;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::core::{
        plugin::FlurxWryCorePlugin,
        bundle::*
    };
}

#[derive(Component, Reflect, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[reflect(Component)]
pub struct WebviewInitialized(());


#[derive(Deref, DerefMut, Default)]
pub(crate) struct WebviewMap(pub HashMap<Entity, wry::WebView>);
