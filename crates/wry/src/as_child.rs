use bevy::app::App;
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::{Bundle, Changed, Commands, Component, Entity, IntoSystemConfigs, MouseButton, NonSend, not, Plugin, Query, Reflect, ReflectComponent, Update};
use bevy::window::{CursorIcon, Window};

use crate::as_child::bounds::Bounds;
use crate::as_child::resize_mode::ResizeMode;
use crate::plugin::WebviewMap;

mod resize_mode;
pub mod bounds;

#[derive(Bundle)]
pub struct AsChild {
    pub parent: ParentWindow,

    pub bounds: Bounds,
    
    pub resizable: Resizable
}


#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct ParentWindow(pub Entity);


#[repr(transparent)]
#[derive(Component, Copy, Clone, Eq, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Resizable(pub bool);

impl Default for Resizable{
    fn default() -> Self {
        Self(true)
    }
}

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
                set_bounds,
            ));
    }
}

fn change_mouse_cursor_icon(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    views: Query<(Entity, &ParentWindow, &Bounds, &Resizable)>,
) {
    for (entity, parent, bounds, resizable) in views.iter() {
        if !resizable.0{
            continue;
        }
        let Ok(mut window) = windows.get_mut(parent.0) else {
            continue;
        };
        let Some(cursor_pos) = window.cursor_position() else {
            continue;
        };
        let cursor_pos = cursor_pos;
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
    mut views: Query<(&mut Bounds, &ResizeMode, &ParentWindow, &Resizable)>,
    window: Query<&Window>,
) {
    for (mut bounds, resize_mode, parent, resizable) in views.iter_mut() {
        if !resizable.0{
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


