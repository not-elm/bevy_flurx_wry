use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};

/// Please see [`wry::WebViewBuilder::with_hotkeys_zoom`].
#[repr(transparent)]
#[derive(Component, Copy, Clone, Reflect, Default, Eq, PartialEq)]
#[reflect(Component, Default)]
pub struct HotkeysZoom(pub bool);

