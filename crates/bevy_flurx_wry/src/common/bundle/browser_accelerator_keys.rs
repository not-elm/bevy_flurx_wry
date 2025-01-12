use bevy::prelude::{Component, ReflectComponent};
use bevy::prelude::{ReflectDefault, Reflect};

/// Please see [`wry::WebViewBuilder::with_browser_accelerator_keys`] for details.
///
/// Default is `true`.
/// 
/// This setting only works on `Windows`. 
#[repr(transparent)]
#[derive(Component, Debug, Eq, PartialEq, Copy, Clone, Reflect, Hash)]
#[reflect(Component, Default)]
pub struct BrowserAcceleratorKeys(pub bool);

impl Default for BrowserAcceleratorKeys{
    fn default() -> Self {
        Self(true)
    }
}