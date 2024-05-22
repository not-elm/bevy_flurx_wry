use bevy::prelude::{Reflect, Resource};

#[derive(Resource, Debug, Copy, Clone, Eq, PartialEq, Hash, Reflect, Default)]
pub struct ApiAllows {
    pub app: AppApiAllows,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Reflect, Default)]
pub struct AppApiAllows {
    pub get_name: bool,
    pub get_version: bool,
    pub exit: bool,
}

impl AppApiAllows {
    pub fn enable_all() -> Self {
        Self {
            get_name: true,
            get_version: true,
            exit: true,
        }
    }
}
