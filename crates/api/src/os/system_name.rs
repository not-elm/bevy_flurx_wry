use crate::macros::api_plugin;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

api_plugin!(
    /// You'll be able to get the system name from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const name: string | null = await window.__FLURX__.os.systemName();
    /// ```
    OsSystemNamePlugin,
    command: system_name
);

#[command(id = "FLURX|os::system_name", internal)]
fn system_name() -> Action<(), Option<String>> {
    once::run(sysinfo::System::name).with(())
}