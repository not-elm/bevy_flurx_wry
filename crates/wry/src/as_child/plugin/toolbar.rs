use bevy::app::{App, Update};
use bevy::input::common_conditions::input_just_released;
use bevy::math::{IVec2, Vec2};
use bevy::prelude::{Commands, Entity, IntoSystemConfigs, MouseButton, NonSend, Plugin, Query, Window, With};
use bevy::winit::WinitWindows;
use mouse_rs::Mouse;

use crate::as_child::bundle::{Bounds, ParentWindow};
use crate::as_child::CurrentMoving;
use crate::prelude::Toolbar;

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app

            .register_type::<Toolbar>()
            .add_systems(Update, (
                move_webview,
                all_remove_current_moving.run_if(input_just_released(MouseButton::Left))
            ));
    }
}

fn move_webview(
    mut views: Query<(&mut Bounds, &ParentWindow, &CurrentMoving), With<CurrentMoving>>,
    winit_windows: NonSend<WinitWindows>,
    windows: Query<&Window>,
) {
    let mouse = Mouse::new();
    let pos = mouse.get_position().unwrap();
    let pos = IVec2::new(pos.x, pos.y).as_vec2();

    for (mut bounds, parent, CurrentMoving(d)) in views.iter_mut() {
        let Ok(window_size) = windows
            .get(parent.0)
            .map(|w| Vec2::new(w.resolution.width(), w.resolution.height())) else {
            continue;
        };
        let Some(winit_window) = winit_windows.get_window(parent.0) else {
            continue;
        };
        let Ok(window_position) = winit_window.inner_position() else {
            continue;
        };
        let window_position = window_position.cast::<f32>();
        let window_position = Vec2::new(window_position.x, window_position.y);
        let cursor_pos = pos - window_position;
        move_bounds(&mut bounds, cursor_pos - *d, window_size);
    }
}

fn all_remove_current_moving(
    mut commands: Commands,
    views: Query<Entity, With<CurrentMoving>>,
) {
    for entity in views.iter() {
        commands.entity(entity).remove::<CurrentMoving>();
    }
}

fn move_bounds(bounds: &mut Bounds, top_left: Vec2, window_size: Vec2) {
    let cursor_pos = top_left.max(Vec2::ZERO);
    let max_pos = window_size - bounds.size;
    bounds.position = cursor_pos.min(max_pos);
}


#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use bevy::utils::default;

    use crate::as_child::plugin::toolbar::move_bounds;
    use crate::prelude::Bounds;

    #[test]
    fn stop_top_left_edge() {
        let mut bounds = new_bounds();
        move_bounds(&mut bounds, Vec2::new(-3., -3.), Vec2::new(100., 100.));
        assert_eq!(bounds.position, Vec2::new(0., 0.));
    }

    #[test]
    fn stop_bottom_right() {
        let mut bounds = new_bounds();
        move_bounds(&mut bounds, Vec2::new(110., 110.), Vec2::new(100., 100.));
        assert_eq!(bounds.position, Vec2::new(95., 90.));
    }

    fn new_bounds() -> Bounds {
        Bounds {
            position: Vec2::new(5., 10.),
            size: Vec2::new(5., 10.),
            ..default()
        }
    }
}