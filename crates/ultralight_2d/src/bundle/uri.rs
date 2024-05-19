use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize, Reflect, Component)]
#[reflect(Component, Default)]
pub enum Load{ 
    Html(String),
    
    Uri(String)
}

impl Default for Load{
    fn default() -> Self {
        Self::Uri("file:///assets/index.html".to_string())
    }
}