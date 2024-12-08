use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_reflect::prelude::{ReflectDefault, Reflect};


/// Please see [`wry::WebViewBuilder::with_user_agent`] for details.
///
/// Default is not specified.
#[repr(transparent)]
#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Default, Reflect)]
#[reflect(Component, Default)]
pub struct UserAgent(pub Option<String>);

impl UserAgent {
    /// Creates the new [`UserAgent`].
    pub fn new(user_agent: impl Into<String>) -> Self{
        Self(Some(user_agent.into()))
    }
}