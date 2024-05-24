use bevy::app::{App, Plugin};
use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::{EventReader, Update};
use ul_next::event::{KeyEvent, KeyEventCreationInfo, KeyEventModifiers, KeyEventType};
use ul_next::key_code::VirtualKeyCode;

use crate::core::plugin::UlViewSystemParam;

pub struct UlKeyboardPlugin;

impl Plugin for UlKeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_key);
    }
}

fn input_key(
    mut er: EventReader<KeyboardInput>,
    param: UlViewSystemParam,
) {
    for e in er.read() {
        param.inspect_focus_views(|view, _, cursor| {
            if cursor.is_none() {
                return;
            }
            view.fire_key_event(KeyEvent::new(KeyEventCreationInfo {
                ty: key_event_type(e),
                modifiers: KeyEventModifiers {
                    alt: false,
                    ctrl: false,
                    meta: false,
                    shift: false,
                },
                virtual_key_code: virtual_keycode(e),
                native_key_code: 0,
                text: text(&e.logical_key),
                unmodified_text: "",
                is_keypad: false,
                is_auto_repeat: false,
                is_system_key: false,
            }).unwrap());
        });
    }
}

fn key_event_type(e: &KeyboardInput) -> KeyEventType {
    match e.state {
        ButtonState::Pressed if matches!(e.logical_key, Key::Character(_)) => KeyEventType::Char,
        ButtonState::Pressed => KeyEventType::KeyDown,
        ButtonState::Released => KeyEventType::KeyUp
    }
}

fn virtual_keycode(e: &KeyboardInput) -> VirtualKeyCode {
    match e.logical_key {
        Key::Backspace => VirtualKeyCode::Back,
        Key::Control => VirtualKeyCode::Control,
        Key::NumLock => VirtualKeyCode::Numlock,
        Key::Shift => VirtualKeyCode::Shift,
        Key::Enter => VirtualKeyCode::Return,
        Key::Tab => VirtualKeyCode::Tab,
        Key::Space => VirtualKeyCode::Space,
        Key::End => VirtualKeyCode::End,
        Key::Home => VirtualKeyCode::Home,
        Key::Clear => VirtualKeyCode::Clear,
        Key::CrSel => VirtualKeyCode::Crsel,
        Key::Delete => VirtualKeyCode::Delete,
        Key::ExSel => VirtualKeyCode::Exsel,
        Key::Insert => VirtualKeyCode::Insert,
        Key::Accept => VirtualKeyCode::Accept,
        Key::Attn => VirtualKeyCode::Attn,
        Key::Escape => VirtualKeyCode::Escape,
        Key::Execute => VirtualKeyCode::Execute,
        Key::Help => VirtualKeyCode::Help,
        Key::Pause => VirtualKeyCode::Pause,
        Key::Play => VirtualKeyCode::Play,
        Key::Select => VirtualKeyCode::Select,
        Key::ZoomIn => VirtualKeyCode::Zoom,
        Key::ZoomOut => VirtualKeyCode::Zoom,
        Key::PrintScreen => VirtualKeyCode::Print,
        Key::Convert => VirtualKeyCode::Convert,
        Key::FinalMode => VirtualKeyCode::Final,
        Key::ModeChange => VirtualKeyCode::Modechange,
        Key::NextCandidate => VirtualKeyCode::Next,
        Key::NonConvert => VirtualKeyCode::Nonconvert,
        Key::Process => VirtualKeyCode::Processkey,
        Key::HangulMode => VirtualKeyCode::Hangul,
        Key::HanjaMode => VirtualKeyCode::Hanja,
        Key::JunjaMode => VirtualKeyCode::Junja,
        Key::KanaMode => VirtualKeyCode::Kana,
        Key::KanjiMode => VirtualKeyCode::Kanji,
        Key::Print => VirtualKeyCode::Print,
        Key::BrowserBack => VirtualKeyCode::BrowserBack,
        Key::BrowserFavorites => VirtualKeyCode::BrowserFavorites,
        Key::BrowserForward => VirtualKeyCode::BrowserForward,
        Key::BrowserHome => VirtualKeyCode::BrowserHome,
        Key::BrowserRefresh => VirtualKeyCode::BrowserRefresh,
        Key::BrowserSearch => VirtualKeyCode::BrowserSearch,
        Key::BrowserStop => VirtualKeyCode::BrowserStop,
        Key::GoBack => VirtualKeyCode::Back,
        Key::GoHome => VirtualKeyCode::Home,
        Key::F1 => VirtualKeyCode::F1,
        Key::F2 => VirtualKeyCode::F2,
        Key::F3 => VirtualKeyCode::F3,
        Key::F4 => VirtualKeyCode::F4,
        Key::F5 => VirtualKeyCode::F5,
        Key::F6 => VirtualKeyCode::F6,
        Key::F7 => VirtualKeyCode::F7,
        Key::F8 => VirtualKeyCode::F8,
        Key::F9 => VirtualKeyCode::F9,
        Key::F10 => VirtualKeyCode::F10,
        Key::F11 => VirtualKeyCode::F11,
        Key::F12 => VirtualKeyCode::F12,
        Key::F13 => VirtualKeyCode::F13,
        Key::F14 => VirtualKeyCode::F14,
        Key::F15 => VirtualKeyCode::F15,
        Key::F16 => VirtualKeyCode::F16,
        Key::F17 => VirtualKeyCode::F17,
        Key::F18 => VirtualKeyCode::F18,
        Key::F19 => VirtualKeyCode::F19,
        Key::F20 => VirtualKeyCode::F20,
        Key::F21 => VirtualKeyCode::F21,
        Key::F22 => VirtualKeyCode::F22,
        Key::F23 => VirtualKeyCode::F23,
        Key::F24 => VirtualKeyCode::F24,
        _ => VirtualKeyCode::Unknown
    }
}


fn text(key: &Key) -> &str {
    match key {
        Key::Character(str) => str.as_str(),
        _ => ""
    }
}

