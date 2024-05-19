use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Sets whether enables clipboard access for the page rendered on **Linux** and **Windows**.
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct EnableClipboard(pub bool);