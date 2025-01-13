use bevy::prelude::{Component, ReflectComponent, ReflectDeserialize, ReflectSerialize};
use bevy::prelude::{Reflect, ReflectDefault};
use serde::{Deserialize, Serialize};

/// Represents a webview theme.
///
/// Default is [`Theme::Auto`].
///
/// This setting only works on `Windows`.
#[derive(Default, Component, Copy, Clone, Eq, PartialEq, Reflect, Serialize, Deserialize)]
#[reflect(Component, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    /// System theme
    #[default]
    Auto,

    /// Light theme
    Light,

    /// Dark theme
    Dark,
}

impl Theme {
    #[cfg(target_os = "windows")]
    pub(crate) fn as_wry_theme(&self) -> wry::Theme {
        match self {
            Theme::Auto => wry::Theme::Auto,
            Theme::Light => wry::Theme::Light,
            Theme::Dark => wry::Theme::Dark
        }
    }
}