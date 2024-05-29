use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};


/// Please see [`wry::WebViewBuilder::with_browser_accelerator_keys`] for details.
/// 
/// Default is `true`.
#[repr(transparent)]
#[derive(Component, Debug, Eq, PartialEq, Copy, Clone, Reflect, Hash)]
#[reflect(Component, Default)]
pub struct BrowserAcceleratorKeys(pub bool);

impl Default for BrowserAcceleratorKeys{
    fn default() -> Self {
        Self(true)
    }
}