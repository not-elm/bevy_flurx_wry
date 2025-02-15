//! Provides mechanism to output the logs.

use bevy::app::Update;
use bevy::log;
use bevy::prelude::{App, EventReader, Plugin};
use bevy_flurx_ipc::ipc_events::IpcEventExt;
use bevy_flurx_ipc::prelude::IpcEvent;
use serde::Deserialize;

/// You will be able to output a massage to the console of the aa process.
///
/// ## Typescript Code Example
///
/// ```ts
/// window.__FLURX__.log.println("message")
/// ```
pub struct AllLogPlugins;

impl Plugin for AllLogPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_ipc_event::<RequestPrintln>("FLURX|log::println")
            .add_ipc_event::<RequestLog>("FLURX|log::log")
            .add_systems(Update, (
                println_event,
                log_event,
            ));
    }
}

#[derive(Deserialize)]
struct RequestPrintln {
    message: String,
}

#[derive(Deserialize)]
struct RequestLog {
    message: String,
    level: RequestLogLevel,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum RequestLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn println_event(mut er: EventReader<IpcEvent<RequestPrintln>>) {
    for event in er.read() {
        println!("{}", event.payload.message);
    }
}

fn log_event(mut er: EventReader<IpcEvent<RequestLog>>) {
    for event in er.read() {
        let message = &event.payload.message;
        match event.payload.level {
            RequestLogLevel::Trace => log::trace!(message),
            RequestLogLevel::Debug => log::debug!(message),
            RequestLogLevel::Info => log::info!(message),
            RequestLogLevel::Warn => log::warn!(message),
            RequestLogLevel::Error => log::error!(message),
        }
    }
}
