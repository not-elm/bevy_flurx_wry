use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault};


/// Custom user-agent for the webview.
#[repr(transparent)]
#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Default, Reflect)]
#[reflect(Component, Default)]
pub struct UserAgent(pub Option<String>);

impl UserAgent {
    pub fn new(user_agent: impl Into<String>) -> Self{
        Self(Some(user_agent.into()))
    }
}