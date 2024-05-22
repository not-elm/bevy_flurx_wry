use bevy::prelude::{Reflect, Resource};

#[derive(Resource, Debug, Copy, Clone, Eq, PartialEq, Hash, Reflect)]
pub struct ApiAllows {
    pub app: AppApiAllows,
}

impl ApiAllows{
    pub fn enable_all() -> Self{
        Self{
           app: AppApiAllows::enable_all() 
        }
    }
}


impl Default for ApiAllows{
    fn default() -> Self {
        Self::enable_all()
    }
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
