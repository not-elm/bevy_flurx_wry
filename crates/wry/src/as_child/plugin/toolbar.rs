use bevy::app::{App, Update};
use bevy::input::common_conditions::input_just_released;
use bevy::math::{IVec2, Vec2};
use bevy::prelude::{Commands, Entity, IntoSystemConfigs, MouseButton, NonSend, Plugin, Query, With};
use bevy::winit::WinitWindows;
use mouse_rs::Mouse;

use crate::as_child::bundle::{Bounds, ParentWindow};
use crate::as_child::CurrentMoving;

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                move_webview,
                remove_drag_move.run_if(input_just_released(MouseButton::Left))
            ));
    }
}

fn move_webview(
    mut views: Query<(&mut Bounds, &ParentWindow, &CurrentMoving), With<CurrentMoving>>,
    windows: NonSend<WinitWindows>,
) {
    let mouse = Mouse::new();
    let pos = mouse.get_position().unwrap();
    let pos = IVec2::new(pos.x, pos.y).as_vec2();

    for (mut bounds, parent, CurrentMoving(d)) in views.iter_mut() {
        let Some(window) = windows.get_window(parent.0) else {
            continue;
        };
        let Ok(window_position) = window.inner_position() else {
            continue;
        };
        let window_position = window_position.cast::<f32>();
        let window_position = Vec2::new(window_position.x, window_position.y);
        let cursor_pos = pos - window_position;
        bounds.position = cursor_pos - *d;
    }
}

fn remove_drag_move(
    mut commands: Commands,
    views: Query<Entity, With<CurrentMoving>>,
) {
    for entity in views.iter() {
        commands.entity(entity).remove::<CurrentMoving>();
    }
}