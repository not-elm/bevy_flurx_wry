use bevy::app::App;
use bevy::prelude::{EventReader, NonSendMut, Plugin, Update};
use tiny_http::{Response, StatusCode};

use bevy_flurx_ipc::ipc_command_queue::{IpcCommand, IpcCommands};
use bevy_flurx_ipc::plugin::IpcResolveEvent;

use crate::core::plugin::UlViewMap;

pub struct IpcPlugin;

impl Plugin for IpcPlugin {
    fn build(&self, app: &mut App) {
        let queue = app.world.resource::<IpcCommands>().clone();
        launch_server(queue);

        app.add_systems(Update, resolve_event);
    }
}

fn launch_server(queue: IpcCommands) {
    std::thread::spawn(move || {
        let server = tiny_http::Server::http("0.0.0.0:9900").unwrap();
        for mut req in server.incoming_requests() {
            let mut buf = String::new();
            if req.as_reader().read_to_string(&mut buf).is_err() {
                req.respond(Response::new_empty(StatusCode::from(200))).unwrap();
                continue;
            }
            let Ok(command) = serde_json::from_str::<IpcCommand>(&buf) else {
                req.respond(Response::new_empty(StatusCode::from(200))).unwrap();
                continue;
            };
            queue.push(command);
            req.respond(Response::new_empty(StatusCode::from(200))).unwrap();
        }
    });
}

fn resolve_event(
    mut er: EventReader<IpcResolveEvent>,
    mut views: NonSendMut<UlViewMap>,
) {
    for IpcResolveEvent {
        entity,
        resolve_id,
        output
    } in er.read() {
        if let Some(view) = views.0.get_mut(entity) {
            view.evaluate_script(&format!("window.__FLURX__.resolveIpc({resolve_id}, {output})"))
                .unwrap()
                .unwrap();
        }
    }
}