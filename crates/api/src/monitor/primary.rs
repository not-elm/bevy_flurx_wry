use crate::macros::api_plugin;
use crate::monitor::{Monitor, PhysicalPosition, PhysicalSize};
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::In;
use bevy_ecs::system::{NonSend, Query};
use bevy_flurx::action::{once, Action};
use bevy_flurx::prelude::OmitInput;
use bevy_flurx_ipc::command;
use bevy_flurx_ipc::component::WebviewEntity;
use bevy_flurx_wry_core::prelude::ParentWindow;
use bevy_winit::WinitWindows;

api_plugin!(
    /// You'll be able to get the current monitor info from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const monitor = await window.__FLURX__.monitor.primary();
    /// ```
    MonitorPrimaryPlugin,
    command: primary
);

#[command(id = "FLURX|monitor::primary", internal)]
fn primary(WebviewEntity(entity): WebviewEntity) -> Action<(), Option<Monitor>> {
    once::run(primary_system).with(entity).omit_input().with(())
}

//noinspection DuplicatedCode
fn primary_system(
    In(entity): In<Entity>,
    parent: Query<&ParentWindow>,
    web_views: NonSend<WinitWindows>,
) -> Option<Monitor> {
    let entity = if let Ok(parent) = parent.get(entity) {
        parent.0
    } else {
        entity
    };

    web_views.get_window(entity)?
        .primary_monitor()
        .map(|m| {
            let size = m.size();
            let p = m.position();
            Monitor {
                name: m.name(),
                size: PhysicalSize {
                    width: size.width,
                    height: size.height,
                },
                position: PhysicalPosition {
                    x: p.x,
                    y: p.y,
                },
                scale_factor: m.scale_factor(),
            }
        })
}