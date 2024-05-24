use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, Event, EventWriter, In, MouseButton, Query, With};
use bevy::window::Window;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;
use serde::Deserialize;

use bevy_flurx_ipc::prelude::WebviewEntity;

use crate::as_child::{DragMove, ParentWindow};

pub fn webview_move_start(
    In(p): In<Pointer>,
    entity: WebviewEntity,
) -> Action<(WebviewEntity, Pointer)> {
    once::run(move_start_system).with((entity, p))
}

pub fn down(
    WebviewEntity(entity): WebviewEntity
) -> Action<(Entity, MouseButton, ButtonState)> {
    once::run(mouse_system).with((entity, MouseButton::Left, ButtonState::Pressed))
}

pub fn up(
    WebviewEntity(entity): WebviewEntity
) -> Action<(Entity, MouseButton, ButtonState)> {
    once::run(mouse_system).with((entity, MouseButton::Left, ButtonState::Released))
}


#[derive(Deserialize, Copy, Clone, Event)]
pub struct Pointer {
    x: f32,
    y: f32,
}

fn move_start_system(
    In((WebviewEntity(entity), pointer)): In<(WebviewEntity, Pointer)>,
    mut commands: Commands,
) {
    commands.entity(entity).insert(DragMove(Vec2::new(pointer.x, pointer.y)));
}

fn mouse_system(
    In((entity, button, state)): In<(Entity, MouseButton, ButtonState)>,
    mut ew: EventWriter<MouseButtonInput>,
    windows: Query<Entity, With<Window>>,
    views: Query<&ParentWindow>,
) {
    let Ok(parent) = views.get(entity) else {
        return;
    };
    if let Ok(window) = windows.get(parent.0) {
        ew.send(MouseButtonInput {
            button,
            state,
            window,
        });
    }
}