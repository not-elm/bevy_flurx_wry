use bevy::prelude::Bundle;
use bevy::sprite::SpriteBundle;

use bevy_flurx_ipc::component::IpcHandlers;

pub use crate::bundle::uri::Load;
pub use crate::bundle::view_size::ViewSize;

mod uri;
mod view_size;


#[derive(Bundle, Default)]
pub struct Ul2DWebViewBundle {
    pub load: Load,

    pub ipc_handlers: IpcHandlers,

    pub size: ViewSize,

    pub sprite: SpriteBundle,
}