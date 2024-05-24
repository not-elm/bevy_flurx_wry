use bevy::app::{App, Update};
use bevy::input::ButtonInput;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::{EventReader, IntoSystemConfigs, MouseButton, on_event, Plugin, Res};
use ul_next::event::{MouseEvent, MouseEventType, ScrollEventType};

use crate::core::plugin::UlViewSystemParam;

pub struct UlMousePlugin;

impl Plugin for UlMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            detect_mouse_move.run_if(on_event::<MouseMotion>()),
            detect_mouse_button,
            detect_mouse_wheel
        ));
    }
}

fn detect_mouse_move(
    param: UlViewSystemParam,
    input: Res<ButtonInput<MouseButton>>,
) {
    param.inspect_focus_views(|view, _, cursor_pos| {
        let mouse_button = ul_mouse_button(&input);
        if let Some(cursor_pos) = cursor_pos {
            view.fire_mouse_event(MouseEvent::new(
                MouseEventType::MouseMoved,
                cursor_pos.x,
                cursor_pos.y,
                mouse_button,
            ).unwrap());
        }
    });
}

fn detect_mouse_button(
    param: UlViewSystemParam,
    input: Res<ButtonInput<MouseButton>>,
) {
    param.inspect_focus_views(|view, is_down, cursor_pos| {
        if let Some(cursor_pos) = cursor_pos {
            let (event_type, button) = if input.just_pressed(MouseButton::Left) {
                (MouseEventType::MouseDown, ul_next::event::MouseButton::Left)
            } else if input.just_released(MouseButton::Left) {
                (MouseEventType::MouseUp, ul_next::event::MouseButton::Left)
            } else if input.just_pressed(MouseButton::Middle) {
                (MouseEventType::MouseDown, ul_next::event::MouseButton::Middle)
            } else if input.just_released(MouseButton::Middle) {
                (MouseEventType::MouseUp, ul_next::event::MouseButton::Middle)
            } else if input.just_pressed(MouseButton::Right) {
                (MouseEventType::MouseDown, ul_next::event::MouseButton::Right)
            } else if input.just_released(MouseButton::Right) {
                (MouseEventType::MouseUp, ul_next::event::MouseButton::Right)
            } else {
                return;
            };

            view.fire_mouse_event(MouseEvent::new(
                event_type,
                cursor_pos.x,
                cursor_pos.y,
                button,
            ).unwrap());
            if matches!(event_type, MouseEventType::MouseDown) {
                is_down.set(button);
            } else {
                is_down.take();
            }
        } else if is_down.is_down() {
            view.fire_mouse_event(MouseEvent::new(
                MouseEventType::MouseUp,
                0,
                0,
                is_down.take().unwrap(),
            ).unwrap());
        }
    });
}

fn detect_mouse_wheel(
    mut er: EventReader<MouseWheel>,
    param: UlViewSystemParam,
) {
    for e in er.read() {
        param.inspect_focus_views(|view, _, _| {
            const LINE_HEIGHT: f32 = 80.;
            let (x, y) = match e.unit {
                // TODO: Need to find the correct way to convert line to pixel.
                MouseScrollUnit::Line => (e.x * LINE_HEIGHT, e.y * LINE_HEIGHT),
                MouseScrollUnit::Pixel => (e.x, e.y)
            };
            view.fire_scroll_event(ul_next::event::ScrollEvent::new(
                ScrollEventType::ScrollByPixel,
                x as i32,
                y as i32,
            ).unwrap());
        });
    }
}

#[inline]
fn ul_mouse_button(input: &ButtonInput<MouseButton>) -> ul_next::event::MouseButton {
    if input.just_pressed(MouseButton::Left) {
        ul_next::event::MouseButton::Left
    } else if input.just_pressed(MouseButton::Middle) {
        ul_next::event::MouseButton::Middle
    } else if input.just_pressed(MouseButton::Right) {
        ul_next::event::MouseButton::Right
    } else {
        ul_next::event::MouseButton::None
    }
}


