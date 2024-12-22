use crate::macros::api_plugin;
use bevy::core::Name;
use bevy::prelude::{Commands, In};
use bevy::window::Window;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;
use bevy_flurx_ipc::command;
use bevy_flurx_wry_core::prelude::{AutoPlay, BrowserAcceleratorKeys, HotkeysZoom, Incognito, InitializeFocused, IsOpenDevtools, Theme, UseDevtools, UseHttpsScheme, WebviewUri, WebviewVisible};
use serde::Deserialize;

api_plugin!(
    /// You'll be able to create the new webview window.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const webWindow = await window.__FLURX__.Webview.create();
    /// ```
    WebWindowCreatePlugin,
    command: create
);

#[derive(Deserialize)]
struct Args {
    identifier: String,
    url: String,
    #[serde(rename = "autoPlay")]
    auto_play: Option<bool>,
    // TODO: works later
    background: Option<String>,
    #[serde(rename = "browserAcceleratorKeys")]
    browser_accelerator_keys: Option<bool>,
    #[serde(rename = "useDevtools")]
    use_devtools: Option<bool>,
    #[serde(rename = "isOpenDevtools")]
    is_open_devtools: Option<bool>,
    #[serde(rename = "initializeFocused")]
    initialize_focused: Option<bool>,
    #[serde(rename = "hotkeysZoom")]
    hotkeys_zoom: Option<bool>,
    #[serde(rename = "useHttpsScheme")]
    use_https_scheme: Option<bool>,
    visible: Option<bool>,
    incognito: Option<bool>,
    #[serde(rename = "userAgent")]
    user_agent: Option<String>,
    theme: Option<Theme>,
}

#[command(id = "FLURX|webWindow::create", internal)]
fn create(In(args): In<Args>) -> Action<Args> {
    once::run(|In(args): In<Args>,
               mut commands: Commands,
    | {
        let mut entity_commands = commands.spawn((
            Window::default(),
            WebviewUri::new(args.url),
            Name::new(args.identifier),
        ));
        if let Some(auto_play) = args.auto_play {
            entity_commands.insert(AutoPlay(auto_play));
        }
        if let Some(incognito) = args.incognito {
            entity_commands.insert(Incognito(incognito));
        }
        if let Some(browser_accelerator_keys) = args.browser_accelerator_keys {
            entity_commands.insert(BrowserAcceleratorKeys(browser_accelerator_keys));
        }
        if let Some(use_devtools) = args.use_devtools {
            entity_commands.insert(UseDevtools(use_devtools));
        }
        if let Some(use_devtools) = args.use_devtools {
            entity_commands.insert(UseDevtools(use_devtools));
        }
        if let Some(is_open_devtools) = args.is_open_devtools {
            entity_commands.insert(IsOpenDevtools(is_open_devtools));
        }
        if let Some(initialize_focused) = args.initialize_focused {
            entity_commands.insert(InitializeFocused(initialize_focused));
        }
        if let Some(hotkeys_zoom) = args.hotkeys_zoom {
            entity_commands.insert(HotkeysZoom(hotkeys_zoom));
        }
        if let Some(use_https_scheme) = args.use_https_scheme {
            entity_commands.insert(UseHttpsScheme(use_https_scheme));
        }
        if let Some(visible) = args.visible {
            entity_commands.insert(WebviewVisible(visible));
        }
        if let Some(theme) = args.theme {
            entity_commands.insert(theme);
        }
    }).with(args)
}