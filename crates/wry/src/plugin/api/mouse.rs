use bevy::input::ButtonState;
use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Entity, Event, EventReader, EventWriter, In, MouseButton, Query, With};
use bevy::window::Window;
use bevy_flurx::action::once;
use bevy_flurx::prelude::{Action, ActionSeed};
use once::run;
use serde::Deserialize;

use bevy_flurx_ipc::prelude::WebviewEntity;

use crate::as_child::{DragMove, ParentWindow};


pub fn webview_move_start(
    WebviewEntity(entity): WebviewEntity
) -> ActionSeed {
    run(move |mut commands: Commands|{
        commands.entity(entity).insert(DragMove);
    })
}

pub fn down(
    WebviewEntity(entity): WebviewEntity
) -> Action<(Entity, MouseButton, ButtonState)> {
    run(mouse_system).with((entity, MouseButton::Left, ButtonState::Pressed))
}

pub fn up(
    WebviewEntity(entity): WebviewEntity
) -> Action<(Entity, MouseButton, ButtonState)> {
    run(mouse_system).with((entity, MouseButton::Left, ButtonState::Released))
}


#[derive(Deserialize, Copy, Clone, Event)]
pub struct Pointer {
    x: f32,
    y: f32,
}


pub fn mouse_move(
    In(p): In<Pointer>,
) -> ActionSeed {
    run(move |mut ew: EventWriter<Pointer>| {
        ew.send(p);
    })
}

pub fn send_mouse_move(
    mut er: EventReader<Pointer>,
    mut ew: EventWriter<MouseMotion>
){
    for p in er.read(){
        ew.send(MouseMotion{
            delta: Vec2::new(p.x, p.y)
        });
    }
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