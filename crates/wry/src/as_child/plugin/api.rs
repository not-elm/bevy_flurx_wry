use bevy::app::App;
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{Added, Commands, Entity, Event, EventWriter, In, MouseButton, Plugin, Query, Update, Window, With};
use bevy_flurx::action::{Action, once};
use serde::Deserialize;

use bevy_flurx_ipc::component::{IpcHandler, IpcHandlers, WebviewEntity};

use crate::as_child::CurrentMoving;
use crate::core::WebviewInitialized;
use crate::prelude::ParentWindow;

pub struct ApiPlugin;

impl Plugin for ApiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, register_api_handlers);
    }
}

fn register_api_handlers(
    mut handlers: Query<&mut IpcHandlers, Added<WebviewInitialized>>,
) {
    for mut handler in handlers.iter_mut() {
        handler.register(IpcHandler::new("FLURX|mouse::webview_move_start", || {
            webview_move_start
        }));
        handler.register(IpcHandler::new("FLURX|mouse::down", || {
            down
        }));
        handler.register(IpcHandler::new("FLURX|mouse::up", || {
            up
        }));
    }
}

fn webview_move_start(
    In(p): In<Pointer>,
    entity: WebviewEntity,
) -> Action<(WebviewEntity, Pointer)> {
    once::run(move_start_system).with((entity, p))
}

fn down(
    WebviewEntity(entity): WebviewEntity
) -> Action<(Entity, MouseButton, ButtonState)> {
    once::run(mouse_system).with((entity, MouseButton::Left, ButtonState::Pressed))
}

fn up(
    WebviewEntity(entity): WebviewEntity
) -> Action<(Entity, MouseButton, ButtonState)> {
    once::run(mouse_system).with((entity, MouseButton::Left, ButtonState::Released))
}

#[derive(Deserialize, Copy, Clone, Event)]
struct Pointer {
    x: f32,
    y: f32,
}

fn move_start_system(
    In((WebviewEntity(entity), pointer)): In<(WebviewEntity, Pointer)>,
    mut commands: Commands,
) {
    commands.entity(entity).insert(CurrentMoving(Vec2::new(pointer.x, pointer.y)));
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