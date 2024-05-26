use bevy::app::{App, PostUpdate, Update};
use bevy::input::common_conditions::{input_just_pressed, input_just_released, input_pressed};
use bevy::math::{IVec2, Rect, Vec2};
use bevy::prelude::{Added, BackgroundColor, ButtonBundle, Changed, Commands, Component, Entity, IntoSystemConfigs, MouseButton, NonSend, Or, Plugin, Query, Reflect, Style, Val, Window, With, Without};
use bevy::ui::{PositionType, ZIndex};
use bevy::utils::default;
use bevy::winit::WinitWindows;
use mouse_rs::Mouse;

use crate::as_child::bundle::{Bounds, ParentWindow};
use crate::as_child::CurrentMoving;
use crate::common::WebviewInitialized;
use crate::prelude::resize::ResizeMode;
use crate::prelude::Toolbar;

pub struct ToolbarPlugin;

impl Plugin for ToolbarPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Toolbar>()
            .add_systems(Update, (
                move_webview.run_if(input_pressed(MouseButton::Left)),
                spawn_toolbar,
                fixed_bounds_position,
                all_remove_current_moving.run_if(input_just_released(MouseButton::Left)),
                grab_toolbar.run_if(input_just_pressed(MouseButton::Left)),
            ))
            .add_systems(PostUpdate, track_toolbar);
    }
}


#[derive(Component, Reflect)]
struct ToolbarLink(Entity);

fn spawn_toolbar(
    mut commands: Commands,
    views: Query<(Entity, &Toolbar, &Bounds), Added<WebviewInitialized>>,
) {
    for (entity, toolbar, bounds) in views.iter() {
        let toolbar_entity = commands
            .spawn(ButtonBundle {
                background_color: BackgroundColor(toolbar.color),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(bounds.position.x.max(0.)),
                    top: Val::Px((bounds.position.y - toolbar.height).max(0.)),
                    width: Val::Px(bounds.size.x),
                    height: Val::Px(toolbar.height),
                    ..default()
                },
                z_index: ZIndex::Global(i32::MAX),
                ..default()
            })
            .id();
        commands.entity(entity).insert(ToolbarLink(toolbar_entity));
    }
}

fn grab_toolbar(
    mut commands: Commands,
    windows: Query<&Window>,
    views: Query<(Entity, &ParentWindow, &Bounds, &Toolbar), Without<ResizeMode>>,
) {
    for (entity, parent, bounds, toolbar) in views.iter() {
        let Ok(window) = windows.get(parent.0) else {
            continue;
        };
        let Some(cursor_pos) = window.cursor_position() else {
            continue;
        };

        if is_grab_toolbar(cursor_pos, bounds, toolbar.height) {
            commands.entity(entity).insert(CurrentMoving(cursor_pos - bounds.position));
            return;
        }
    }
}

fn move_webview(
    mut views: Query<(&mut Bounds, &ParentWindow, &CurrentMoving, Option<&Toolbar>), With<CurrentMoving>>,
    winit_windows: NonSend<WinitWindows>,
    windows: Query<&Window>,
) {
    let mouse = Mouse::new();
    let pos = mouse.get_position().unwrap();
    let pos = IVec2::new(pos.x, pos.y).as_vec2();

    for (mut bounds, parent, CurrentMoving(d), toolbar) in views.iter_mut() {
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
        move_bounds(&mut bounds, cursor_pos - *d, window_size, toolbar.map(|t| t.height));
    }
}

fn track_toolbar(
    mut toolbars: Query<&mut Style>,
    views: Query<(&Bounds, &Toolbar, &ToolbarLink), Or<(Changed<Bounds>, Changed<Toolbar>)>>,
) {
    for (bounds, toolbar, link) in views.iter() {
        if let Ok(mut style) = toolbars.get_mut(link.0) {
            style.width = Val::Px(bounds.size.x);
            style.height = Val::Px(toolbar.height);
            style.left = Val::Px(bounds.position.x);
            style.top = Val::Px(bounds.position.y - toolbar.height);
        }
    }
}

fn fixed_bounds_position(
    mut views: Query<(&mut Bounds, &Toolbar)>
) {
    for (mut bounds, toolbar) in views.iter_mut() {
        if bounds.position.y < toolbar.height {
            bounds.position.y = toolbar.height;
        }
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

fn is_grab_toolbar(
    cursor_pos: Vec2,
    bounds: &Bounds,
    toolbar_height: f32,
) -> bool {
    let toolbar_half_size = Vec2::new(bounds.size.x, toolbar_height) / 2.;
    let center = Vec2::new(bounds.position.x + toolbar_half_size.x, bounds.position.y - toolbar_half_size.y);
    let toolbar_rect = Rect::from_center_half_size(center, toolbar_half_size);
    toolbar_rect.contains(cursor_pos)
}


#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use bevy::utils::default;

    use crate::as_child::plugin::toolbar::{is_grab_toolbar, move_bounds};
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

    #[test]
    fn grab_toolbar() {
        let cursor_pos = Vec2::new(110., 90.);
        let bounds = Bounds {
            position: Vec2::new(90., 110.),
            size: Vec2::new(100., 100.),
            ..default()
        };
        const TOOLBAR_HEIGHT: f32 = 30.;
        assert!(is_grab_toolbar(cursor_pos, &bounds, TOOLBAR_HEIGHT));
    }

    #[test]
    fn not_grab_toolbar_cursor_in_webview() {
        let cursor_pos = Vec2::new(110., 120.);
        let bounds = Bounds {
            position: Vec2::new(90., 110.),
            size: Vec2::new(100., 100.),
            ..default()
        };
        const TOOLBAR_HEIGHT: f32 = 30.;
        assert!(!is_grab_toolbar(cursor_pos, &bounds, TOOLBAR_HEIGHT));
    }

    #[test]
    fn not_grab_toolbar_cursor_outside_toolbar() {
        let cursor_pos = Vec2::new(80., 100.);
        let bounds = Bounds {
            position: Vec2::new(90., 110.),
            size: Vec2::new(100., 100.),
            ..default()
        };
        const TOOLBAR_HEIGHT: f32 = 30.;
        assert!(!is_grab_toolbar(cursor_pos, &bounds, TOOLBAR_HEIGHT));
    }

    fn new_bounds() -> Bounds {
        Bounds {
            position: Vec2::new(5., 10.),
            size: Vec2::new(5., 10.),
            ..default()
        }
    }
}