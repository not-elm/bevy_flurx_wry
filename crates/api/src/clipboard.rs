//! Provides mechanism to write and read the system clipboard.

use bevy_app::{PluginGroup, PluginGroupBuilder};
use crate::error::ApiResult;
use bevy_ecs::prelude::In;
use bevy_flurx::action::Action;
use bevy_flurx::action::once;
use bevy_flurx_ipc::command;
use crate::macros::define_api_plugin;


/// Allows you to use all clipboard plugins.
///
/// ## Plugins
///
/// - [ClipboardGetTextPlugin]
/// - [ClipboardSetTextPlugin]
pub struct ClipboardPlugins;
impl PluginGroup for ClipboardPlugins{
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ClipboardSetTextPlugin)
            .add(ClipboardGetTextPlugin)
    }
}

define_api_plugin!(
    /// You'll be able to set a text to the system clipboard.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// await window.__FLURX__.clipboard.setText("hello world!");
    /// ```
    ClipboardSetTextPlugin,
    command: set_text
);

define_api_plugin!(
    /// You'll be able to get a text from the system clipboard.
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const text: string = await window.__FLURX__.clipboard.getText();
    /// ```
    ClipboardGetTextPlugin,
    command: get_text
);

#[command(id="FLURX|clipboard::set_text", internal)]
fn set_text(In(args): In<String>) -> Action<String, ApiResult>{
    once::run(set_text_system).with(args)
}

#[command(id="FLURX|clipboard::get_text", internal)]
fn get_text() -> Action<(), ApiResult<String>>{
    once::run(get_text_system).with(())
}

fn set_text_system(
    In(text): In<String>
) -> ApiResult {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}

fn get_text_system() -> ApiResult<String> {
    let mut clipboard = arboard::Clipboard::new()?;
    Ok(clipboard.get_text()?)
}


#[cfg(test)]
//noinspection DuplicatedCode
mod tests {
    use crate::tests::test_app;
    use bevy_app::{Startup, Update};
    use bevy_ecs::prelude::Commands;
    use bevy_flurx::action::once;
    use bevy_flurx::prelude::{Reactor, Then};
    use crate::clipboard::{get_text_system, set_text_system};

    #[test]
    fn test_clipboard() {
        let mut app = test_app();
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Reactor::schedule(|task| async move {
                let clipboard_text: String = task.will(Update, {
                    once::run(set_text_system).with("Hello!".to_string())
                        .then(once::run(get_text_system))
                })
                    .await
                    .unwrap();
                assert_eq!(clipboard_text, "Hello!");
            }));
        });
        app.update();
    }
}