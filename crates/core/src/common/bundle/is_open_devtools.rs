use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Represents whether the webview devtools should be used.
///
/// This component and [`UseDevtools`](crate::prelude::UseDevtools) must be `true` if you want to open developer tools.
#[derive(Component, Clone, Debug, Eq, PartialEq, Hash, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub struct IsOpenDevtools(pub bool);