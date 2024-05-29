use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};

/// Please see [`wry::WebViewBuilder::with_hotkeys_zoom`] for details.
/// 
/// Default is `true`.
#[repr(transparent)]
#[derive(Component, Copy, Clone, Reflect, Eq, PartialEq)]
#[reflect(Component, Default)]
pub struct HotkeysZoom(pub bool);

impl Default for HotkeysZoom{
    fn default() -> Self {
        Self(true)
    }
}