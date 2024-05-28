use bevy::app::{App, Update};
use bevy::input::common_conditions::input_just_released;
use bevy::math::{IVec2, Vec2};
use bevy::prelude::{Added, Changed, Commands, Entity, In, IntoSystemConfigs, MouseButton, NonSend, Plugin, Query, Window, With};
use bevy::winit::WinitWindows;
use bevy_flurx::action::once;
use bevy_flurx::prelude::{ActionSeed, Omit};
use mouse_rs::Mouse;
use serde::Deserialize;

use bevy_flurx_ipc::component::WebviewEntity;
use bevy_flurx_ipc::prelude::IpcHandlers;

use crate::as_child::bundle::{Bounds, ParentWindow};
use crate::as_child::CurrentMoving;
use crate::common::WebviewInitialized;
use crate::prelude::{EventEmitter, GripZone};

pub struct GripZonePlugin;

impl Plugin for GripZonePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                move_webview,
                all_remove_current_moving.run_if(input_just_released(MouseButton::Left)),
                register_api,
                resize_grip_zone
            ));
    }
}

fn register_api(
    mut views: Query<&mut IpcHandlers, Added<WebviewInitialized>>
) {
    for mut handlers in views.iter_mut() {
        handlers.register(grip_zone_grab());
        handlers.register(grip_zone_release());
    }
}

fn resize_grip_zone(
    mut views: Query<(&mut EventEmitter, &GripZone), Changed<GripZone>>
) {
    for (mut emitter, grip_zone) in views.iter_mut() {
        emitter.emit("FLURX|grip::resize", grip_zone.0);
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
        move_bounds(&mut bounds, cursor_pos - *d, window_size, None);
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

fn move_bounds(bounds: &mut Bounds, top_left: Vec2, window_size: Vec2, toolbar_height: Option<f32>) {
    let max = toolbar_height.map(|height| Vec2::new(0., height)).unwrap_or_default();
    let cursor_pos = top_left.max(max);
    let max_pos = (window_size - bounds.size).max(Vec2::ZERO);
    bounds.position = cursor_pos.min(max_pos);
}

#[bevy_flurx_ipc::command(id = "FLURX|grip::grab")]
fn grip_zone_grab(In(pos): In<CursorPos>, entity: WebviewEntity) -> ActionSeed {
    fn grab(
        In((pos, entity)): In<(CursorPos, WebviewEntity)>,
        mut commands: Commands,
    ) {
        commands.entity(entity.0).insert(CurrentMoving(Vec2::new(pos.x, pos.y)));
    }
    once::run(grab).with((pos, entity)).omit()
}

#[bevy_flurx_ipc::command(id = "FLURX|grip::release")]
fn grip_zone_release(entity: WebviewEntity) -> ActionSeed {
    once::run(move |mut commands: Commands| {
        commands.entity(entity.0).remove::<CurrentMoving>();
    }).omit()
}

#[derive(Deserialize)]
struct CursorPos {
    x: f32,
    y: f32,
}


#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use bevy::utils::default;

    use crate::as_child::plugin::grip_zone::move_bounds;
    use crate::prelude::Bounds;

    #[test]
    fn stop_top_left_edge() {
        let mut bounds = new_bounds();
        move_bounds(&mut bounds, Vec2::new(-3., -3.), Vec2::new(100., 100.), None);
        assert_eq!(bounds.position, Vec2::new(0., 0.));
    }

    #[test]
    fn stop_top_left_edge_with_toolbar() {
        let mut bounds = new_bounds();
        move_bounds(&mut bounds, Vec2::new(-3., -3.), Vec2::new(100., 100.), Some(10.));
        assert_eq!(bounds.position, Vec2::new(0., 10.));
    }

    #[test]
    fn stop_bottom_right() {
        let mut bounds = new_bounds();
        move_bounds(&mut bounds, Vec2::new(110., 110.), Vec2::new(100., 100.), None);
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