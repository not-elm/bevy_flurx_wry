use bevy::prelude::{Component, ReflectComponent};
use bevy::prelude::{ReflectDefault, Reflect};
use serde::{Deserialize, Serialize};

/// Represents whether the webview devtools should be used.
/// 
/// If `true`, pressing `F12` will open developer tools.
/// 
/// Default is `false`.
#[repr(transparent)]
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct UseDevtools(pub bool);