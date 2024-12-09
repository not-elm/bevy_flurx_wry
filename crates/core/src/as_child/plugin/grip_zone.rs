use crate::as_child::bundle::{Bounds, ParentWindow};
use crate::as_child::CurrentMoving;
use crate::common::WryWebViews;
use crate::prelude::{DragEntered, EventEmitter, GripZone};
#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::util::WryResultLog;
use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::{
    on_event, Changed, Commands, Condition, Entity, EventReader, IntoSystemConfigs, NonSend, Query,
    With,
};
use bevy_flurx_ipc::ipc_events::{IpcEvent, IpcEventExt};
use bevy_input::common_conditions::input_just_released;
use bevy_input::mouse::MouseButton;
use bevy_math::{IVec2, Vec2};
use bevy_window::Window;
use bevy_winit::WinitWindows;
use mouse_rs::Mouse;
use serde::Deserialize;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use wry::raw_window_handle::RawWindowHandle;
use wry::raw_window_handle::HasWindowHandle;

pub struct GripZonePlugin;

impl Plugin for GripZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_ipc_event::<OnGripGrab>("FLURX|grip::grab")
            .add_ipc_event::<OnGripRelease>("FLURX|grip::release")
            .add_systems(
                Update,
                (
                    move_webview,
                    all_remove_current_moving
                        .run_if(input_just_released(MouseButton::Left).or(on_event::<DragEntered>)),
                    resize_grip_zone,
                    grip_zone_grab,
                    grip_zone_release,
                ),
            );
    }
}

fn resize_grip_zone(mut views: Query<(&mut EventEmitter, &GripZone), Changed<GripZone>>) {
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
    let Ok(pos) = mouse.get_position() else {
        return;
    };
    let pos = IVec2::new(pos.x, pos.y).as_vec2();

    for (mut bounds, parent, CurrentMoving(d)) in views.iter_mut() {
        let Ok(window_size) = windows
            .get(parent.0)
            .map(|w| Vec2::new(w.resolution.width(), w.resolution.height()))
        else {
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

fn all_remove_current_moving(mut commands: Commands, views: Query<Entity, With<CurrentMoving>>) {
    for entity in views.iter() {
        commands.entity(entity).remove::<CurrentMoving>();
    }
}

fn move_bounds(
    bounds: &mut Bounds,
    top_left: Vec2,
    window_size: Vec2,
    toolbar_height: Option<f32>,
) {
    let max = toolbar_height
        .map(|height| Vec2::new(0., height))
        .unwrap_or_default();
    let cursor_pos = top_left.max(max);
    let max_pos = (window_size - bounds.size).max(Vec2::ZERO);
    bounds.position = cursor_pos.min(max_pos);
}

#[derive(Deserialize)]
struct OnGripGrab {
    x: f32,
    y: f32,
}

fn grip_zone_grab(
    mut er: EventReader<IpcEvent<OnGripGrab>>,
    mut commands: Commands,
    webviews: NonSend<WryWebViews>,
    winit_windows: NonSend<WinitWindows>,
    views: Query<&ParentWindow>,
) {
    for event in er.read() {
        let pos = &event.payload;
        commands
            .entity(event.webview_entity)
            .insert(CurrentMoving(Vec2::new(pos.x, pos.y)));
        let Some(_webview) = webviews.0.get(&event.webview_entity) else {
            continue;
        };
        let Some(window_handle) = views
            .get(event.webview_entity)
            .ok()
            .and_then(|p| winit_windows.get_window(p.0))
            .and_then(|w| w.window_handle().ok())
            .map(|h| h.as_raw())
        else {
            continue;
        };
        match window_handle {
            #[cfg(target_os = "windows")]
            RawWindowHandle::Win32(handle) => {
                use wry::WebViewExtWindows;
                _webview.reparent(handle.hwnd.get()).output_log_if_failed();
            }
            #[cfg(target_os = "macos")]
            RawWindowHandle::AppKit(_) => {
                {
                    use wry::WebViewExtMacOS;
                    _webview.reparent(weview.ns_window()).output_log_if_failed();
                }
            }
            _ => {}
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct OnGripRelease {
    __FLURX__grip_release: u8,
}

fn grip_zone_release(mut er: EventReader<IpcEvent<OnGripRelease>>, mut commands: Commands) {
    for IpcEvent { webview_entity, .. } in er.read() {
        commands.entity(*webview_entity).remove::<CurrentMoving>();
    }
}

#[cfg(test)]
mod tests {
    use bevy_math::Vec2;
    use bevy_utils::default;

    use crate::as_child::plugin::grip_zone::move_bounds;
    use crate::prelude::Bounds;

    #[test]
    fn stop_top_left_edge() {
        let mut bounds = new_bounds();
        move_bounds(
            &mut bounds,
            Vec2::new(-3., -3.),
            Vec2::new(100., 100.),
            None,
        );
        assert_eq!(bounds.position, Vec2::new(0., 0.));
    }

    #[test]
    fn stop_top_left_edge_with_toolbar() {
        let mut bounds = new_bounds();
        move_bounds(
            &mut bounds,
            Vec2::new(-3., -3.),
            Vec2::new(100., 100.),
            Some(10.),
        );
        assert_eq!(bounds.position, Vec2::new(0., 10.));
    }

    #[test]
    fn stop_bottom_right() {
        let mut bounds = new_bounds();
        move_bounds(
            &mut bounds,
            Vec2::new(110., 110.),
            Vec2::new(100., 100.),
            None,
        );
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
