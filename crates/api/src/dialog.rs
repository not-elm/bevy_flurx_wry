use crate::macros::define_api_plugin;
use bevy_app::{PluginGroup, PluginGroupBuilder};
use bevy_ecs::system::In;
use bevy_flurx::action::once;
use bevy_flurx::prelude::Action;
use bevy_flurx_ipc::command;
use rfd::{MessageDialogResult, MessageLevel};
use serde::Deserialize;

/// Allows you to use all dialog plugins
///
/// ## Plugins
///
/// - [DialogAskPlugin]
pub struct AllDialogPlugins;
impl PluginGroup for AllDialogPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DialogAskPlugin)
    }
}

define_api_plugin!(
    /// You will be able to control a dialog for asking from typescript(or js).
    ///
    /// ## Typescript Code Example
    ///
    /// ```ts
    /// const yes: bool = await window.__FLURX__.dialog.ask("question");
    /// ```
    DialogAskPlugin,
    command: ask
);

#[derive(Default, Deserialize)]
struct Args {
    #[serde(rename = "questionMessage")]
    question_message: String,
    title: Option<String>,
    level: Option<DialogLevel>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum DialogLevel {
    Info,
    Warn,
    Error,
}

impl DialogLevel {
    fn to_message_level(&self) -> MessageLevel {
        match self {
            Self::Info => MessageLevel::Info,
            Self::Warn => MessageLevel::Warning,
            Self::Error => MessageLevel::Error,
        }
    }
}

#[command(id = "FLURX|dialog::ask", internal)]
fn ask(In(args): In<Args>) -> Action<Args, bool> {
    once::run(ask_system).with(args)
}

fn ask_system(
    In(args): In<Args>,
) -> bool {
    let mut dialog = rfd::MessageDialog::new();
    if let Some(title) = args.title {
        dialog = dialog.set_title(title);
    }
    if let Some(dialog_type) = args.level {
        dialog = dialog.set_level(dialog_type.to_message_level());
    }
    let dialog_result = dialog
        .set_description(args.question_message)
        .set_buttons(rfd::MessageButtons::YesNo)
        .show();
    matches!(dialog_result, MessageDialogResult::Ok | MessageDialogResult::Yes)
}