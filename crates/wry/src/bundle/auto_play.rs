use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};
use serde::{Deserialize, Serialize};

///  Sets whether all media can be played without user interaction.
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct AutoPlay(pub bool);