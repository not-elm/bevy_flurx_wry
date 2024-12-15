use crate::macros::api_plugin;
use bevy_flurx::action::{once, Action};
use bevy_flurx_ipc::command;

api_plugin!(
    /// You'll be able to get  the preferred locale for the system from a webview.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const locale: string | null = await window.__FLURX__.os.locale();
    /// ```
    OsLocalePlugin,
    command: locale
);

#[command(id = "FLURX|os::locale", internal)]
fn locale() -> Action<(), Option<String>> {
    once::run(sys_locale::get_locale).with(())
}