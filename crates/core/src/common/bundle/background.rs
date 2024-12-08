use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::prelude::{ReflectDefault, Reflect};
use serde::{Deserialize, Serialize};
use bevy_color::Color;

/// Represents the webview background. 
/// 
/// Default is [`Background::Unspecified`].
#[derive(Component, Clone, Debug, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default)]
pub enum Background {
    /// Default settings will be applied.
    #[default]
    Unspecified,

    /// Sets the specified background color.
    Color(Color),

    /// Sets the webView should be transparent.
    Transparent,
}