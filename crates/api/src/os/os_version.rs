use crate::macros::api_plugin;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

api_plugin!(
    /// You'll be able to get the system version from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.os.version();
    /// ```
    OsVersionPlugin,
    command: os_version
);

#[command(id = "FLURX|os::os_version", internal)]
fn os_version() -> Action<(), Option<String>> {
    fn f() -> Option<String> {
        sysinfo::System::os_version()
    }

    once::run(f).with(())
}