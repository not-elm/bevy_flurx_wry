use std::path::Path;
use std::sync::{Arc, Mutex};

use bevy::app::{App, Plugin};
use bevy::ecs::system::SystemParam;
use bevy::math::{IVec2, Rect, UVec2, Vec2};
use bevy::prelude::{Camera, Component, Entity, GlobalTransform, NonSend, Query, Transform, Window, With};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use ul_next::{Config, platform, Renderer, View};

use bevy_flurx_ipc::plugin::FlurxIpcPlugin;

use crate::core::plugin::ipc::IpcPlugin;
use crate::core::plugin::keyboard::UlKeyboardPlugin;
use crate::core::plugin::mouse::UlMousePlugin;
use crate::core::plugin::render::RenderPlugin;
use crate::core::plugin::setup_view::SetupViewPlugin;

mod mouse;
mod render;
mod ipc;
mod setup_view;
mod keyboard;


pub struct FlurxUlPlugin {
    pub file_system_base_dir: String,
    pub log_path: String,
}

impl Plugin for FlurxUlPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<FlurxIpcPlugin>() {
            app.add_plugins(FlurxIpcPlugin);
        }
        app.init_non_send_resource::<UlViewMap>();

        app.add_plugins((
            UlMousePlugin,
            UlKeyboardPlugin,
            SetupViewPlugin,
            IpcPlugin,
            RenderPlugin,
        ));

        platform::enable_platform_fontloader();
        platform::enable_platform_filesystem(&self.file_system_base_dir).unwrap();
        let log_path = Path::new(&self.log_path);
        if let Some(dir) = log_path.parent() {
            if !dir.exists() {
                std::fs::create_dir_all(dir).unwrap();
            }
        }
        platform::enable_default_logger(&self.log_path).expect("failed initialize logger");

        let config = Config::start()

            .build().unwrap();
        let renderer = Renderer::create(config).unwrap();
        app.insert_non_send_resource(renderer);
    }
}

impl Default for FlurxUlPlugin {
    fn default() -> Self {
        Self {
            file_system_base_dir: "./assets".to_string(),
            log_path: "log/ul.log".to_string(),
        }
    }
}

#[repr(transparent)]
#[derive(Default)]
struct UlViewMap(HashMap<Entity, View>);


#[derive(Component, Default)]
struct DownKey(Arc<Mutex<Option<ul_next::event::MouseButton>>>);

impl DownKey {
    pub fn is_down(&self) -> bool {
        self.0.lock().unwrap().is_some()
    }

    pub fn set(&self, button: ul_next::event::MouseButton) {
        self.0.lock().unwrap().replace(button);
    }

    pub fn take(&self) -> Option<ul_next::event::MouseButton> {
        self.0.lock().unwrap().take()
    }
}


#[derive(SystemParam)]
struct UlViewSystemParam<'w, 's> {
    window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    view_map: NonSend<'w, UlViewMap>,
    web_views: Query<'w, 's, (&'static DownKey, &'static Transform)>,
    camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform)>,
}

impl<'w, 's> UlViewSystemParam<'w, 's> {
    pub(crate) fn inspect_focus_views(&self, f: impl Fn(&View, &DownKey, Option<IVec2>)) {
        let window = self.window.single();
        let Some(window_cursor_position) = window.cursor_position() else {
            return;
        };
        let (camera, camera_transform) = self.camera.single();

        for (entity, view) in self.view_map.0.iter() {
            let cursor_pos = self.mouse_cursor_position_on_view(
                camera,
                camera_transform,
                window_cursor_position,
                view,
                *entity,
            );
            let Ok((is_down, _)) = self.web_views.get(*entity) else {
                continue;
            };
            f(view, is_down, cursor_pos)
        }
    }

    fn mouse_cursor_position_on_view(
        &self,
        camera: &Camera,
        camera_transform: &GlobalTransform,
        window_cursor_pos: Vec2,
        view: &View,
        view_entity: Entity,
    ) -> Option<IVec2> {
        let (_, tf) = self.web_views.get(view_entity).ok()?;
        let size = UVec2::new(view.width(), view.height()).as_vec2();
        let pos = camera.world_to_viewport(camera_transform, tf.translation)?;
        let view_rect = Rect::from_center_size(pos, size);

        if view_rect.contains(window_cursor_pos) {
            let pos = (window_cursor_pos - view_rect.min).as_ivec2();
            Some(pos)
        } else {
            None
        }
    }
}