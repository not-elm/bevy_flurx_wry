use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};
use serde::{Deserialize, Serialize};


/// This is a marker component that indicates a webview is in a uninitialized state.
/// 
/// It will be removed after initialization. 
#[derive(Component, Default, Reflect, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct WebviewUninitialized;
