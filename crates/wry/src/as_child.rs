use bevy::app::{App, PostUpdate};
use bevy::input::common_conditions::{input_just_released, input_pressed};
use bevy::math::{IVec2, Vec2};
use bevy::prelude::{Bundle, Changed, Commands, Component, Entity, IntoSystemConfigs, MouseButton, NonSend, not, Plugin, Query, Reflect, ReflectComponent, Update, With, Without};
use bevy::window::{CursorIcon, Window};
use bevy::winit::WinitWindows;
use mouse_rs::Mouse;

use crate::as_child::bounds::Bounds;
use crate::as_child::resize_mode::ResizeMode;
use crate::plugin::WebviewMap;

mod resize_mode;
pub mod bounds;

#[derive(Bundle)]
pub struct AsChild {
    pub parent: ParentWindow,

    pub bounds: Bounds,

    pub resizable: Resizable,
}


#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct ParentWindow(pub Entity);


#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Resizable(pub bool);

impl Default for Resizable {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Component)]
pub(crate) struct DragMove(pub Vec2);


pub struct AsChildPlugin;

impl Plugin for AsChildPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ParentWindow>()
            .register_type::<Bounds>()
            .register_type::<ResizeMode>()
            .register_type::<Resizable>()
            .add_systems(Update, (
                resize.run_if(input_pressed(MouseButton::Left)),
                change_mouse_cursor_icon.run_if(not(input_pressed(MouseButton::Left))),
                move_webview,
                remove_drag_move.run_if(input_just_released(MouseButton::Left))
            ))
            .add_systems(PostUpdate, set_bounds);
    }
}

fn change_mouse_cursor_icon(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    views: Query<(Entity, &ParentWindow, &Bounds, &Resizable), Without<DragMove>>,
) {
    for (entity, parent, bounds, resizable) in views.iter() {
        if !resizable.0 {
            continue;
        }
        let Ok(mut window) = windows.get_mut(parent.0) else {
            continue;
        };
        let Some(cursor_pos) = window.cursor_position() else {
            continue;
        };
        if let Some(resize_mode) = bounds.maybe_resizable(cursor_pos) {
            commands.entity(entity).insert(resize_mode);
            window.cursor.icon = resize_mode.cursor_icon();
        } else {
            commands.entity(entity).remove::<ResizeMode>();
            window.cursor.icon = CursorIcon::Default;
        }
    }
}

fn resize(
    mut views: Query<(&mut Bounds, &ResizeMode, &ParentWindow, &Resizable), Without<DragMove>>,
    window: Query<&Window>,
) {
    for (mut bounds, resize_mode, parent, resizable) in views.iter_mut() {
        if !resizable.0 {
            continue;
        }
        let Ok(window) = window.get(parent.0) else {
            continue;
        };
        if let Some(cursor_pos) = window.cursor_position() {
            bounds.resize(resize_mode, cursor_pos);
        }
    }
}

fn set_bounds(
    webview_map: NonSend<WebviewMap>,
    views: Query<(Entity, &Bounds), Changed<Bounds>>,
) {
    for (entity, bounds) in views.iter() {
        if let Some(webview) = webview_map.0.get(&entity) {
            let _ = webview.set_bounds(bounds.as_wry_rect());
        }
    }
}

fn move_webview(
    mut views: Query<(&mut Bounds, &ParentWindow, &DragMove), With<DragMove>>,
    windows: NonSend<WinitWindows>,
) {
    let mouse = Mouse::new();
    let pos = mouse.get_position().unwrap();
    let pos = IVec2::new(pos.x, pos.y).as_vec2();

    for (mut bounds, parent, DragMove(d)) in views.iter_mut() {
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
    views: Query<Entity, With<DragMove>>,
) {
    for entity in views.iter() {
        commands.entity(entity).remove::<DragMove>();
    }
}
