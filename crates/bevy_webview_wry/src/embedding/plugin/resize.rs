use crate::common::{WebviewInitialized, WryWebViews};
use crate::embedding::bundle::resize::ResizeMode;
use crate::embedding::bundle::{Bounds, EmbedWithin, Resizable};
use crate::embedding::CurrentMoving;
use bevy::input::common_conditions::input_pressed;
use bevy::math::Vec2;
use bevy::prelude::{
    not, Added, App, Changed, Commands, Entity, IntoSystemConfigs, MouseButton, NonSend, Or,
    Plugin, Query, Update, Without,
};
use bevy::window::Window;
use bevy::winit::cursor::CursorIcon;

pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ResizeMode>().add_systems(
            Update,
            (
                change_mouse_cursor_icon.run_if(not(input_pressed(MouseButton::Left))),
                resize_bounds.run_if(input_pressed(MouseButton::Left)),
                render_bounds,
            ),
        );
    }
}

fn change_mouse_cursor_icon(
    mut commands: Commands,
    mut windows: Query<(Entity, &Window)>,
    views: Query<(Entity, &EmbedWithin, &Bounds, &Resizable), Without<CurrentMoving>>,
) {
    for (entity, parent, bounds, resizable) in views.iter() {
        if !resizable.0 {
            continue;
        }
        let Ok((window_entity, window)) = windows.get_mut(parent.0) else {
            continue;
        };
        let Some(cursor_pos) = window.cursor_position() else {
            continue;
        };
        let cursor_pos = if cfg!(target_os = "macos") {
            Vec2::new(cursor_pos.x, window.height() - cursor_pos.y)
        } else {
            cursor_pos
        };
        if let Some(resize_mode) = bounds.maybe_resizable(cursor_pos, None) {
            commands.entity(entity).insert(resize_mode);
            commands
                .entity(window_entity)
                .insert(CursorIcon::System(resize_mode.cursor_icon()));
        } else {
            commands.entity(entity).remove::<ResizeMode>();
            commands.entity(window_entity).insert(CursorIcon::default());
        }
    }
}

fn resize_bounds(
    mut views: Query<(&mut Bounds, &ResizeMode, &EmbedWithin, &Resizable), Without<CurrentMoving>>,
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
            let cursor_pos = if cfg!(target_os = "macos") {
                Vec2::new(cursor_pos.x, window.height() - cursor_pos.y)
            } else {
                cursor_pos
            };
            bounds.transform(resize_mode, cursor_pos, 0.);
        }
    }
}

fn render_bounds(
    webview_map: NonSend<WryWebViews>,
    views: Query<(Entity, &Bounds), Or<(Changed<Bounds>, Added<WebviewInitialized>)>>,
) {
    for (entity, bounds) in views.iter() {
        if let Some(webview) = webview_map.0.get(&entity) {
            if let Err(e) = webview.set_bounds(bounds.as_wry_rect()) {
                bevy::log::error!("Failed to set resize bounds: {}", e);
            }
        }
    }
}
