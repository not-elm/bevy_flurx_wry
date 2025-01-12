use bevy::prelude::{Component, ReflectComponent};
use bevy::prelude::{Reflect, ReflectDefault};


/// Please see [`wry::WebViewBuilder::with_initialization_script`] for details.
#[repr(transparent)]
#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Default, Reflect)]
#[reflect(Component, Default)]
pub struct InitializationScripts(Vec<String>);

impl InitializationScripts {
    /// Creates the new [`InitializationScripts`].
    pub fn new<S: Into<String>>(scripts: impl IntoIterator<Item=S>) -> Self {
        Self(scripts.into_iter().map(S::into).collect())
    }

    /// Appends the initialization script.
    pub fn append<>(&mut self, script: impl Into<String>) -> &mut Self {
        self.0.push(script.into());
        self
    }

    pub(crate) fn to_scripts(&self) -> String {
        self.0.join(";")
    }
}

