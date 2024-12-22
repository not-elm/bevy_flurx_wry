//! Provides mechanism to output the logs.

use bevy::prelude::{App, EventReader, Plugin, PostUpdate};
use serde::Deserialize;
use bevy_flurx_ipc::ipc_events::IpcEventExt;
use bevy_flurx_ipc::prelude::IpcEvent;

/// You will be able to output a massage to the console of the core process.
///
/// ## Typescript Code Example
///
/// ```ts
/// window.__FLURX__.log.println("message")
/// ```
pub struct LogPrintlnApiPlugin;

impl Plugin for LogPrintlnApiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_ipc_event::<RequestPrintln>("FLURX|log::println")
            .add_systems(PostUpdate, println_event);
    }
}

#[derive(Deserialize)]
struct RequestPrintln {
    message: String,
}

fn println_event(mut er: EventReader<IpcEvent<RequestPrintln>>) {
    for e in er.read() {
        println!("{}", e.payload.message);
    }
}
