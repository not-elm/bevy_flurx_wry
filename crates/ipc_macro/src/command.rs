mod async_command;
mod action_command;

use crate::command::action_command::expand_action_command;
use crate::command::async_command::expand_async_command;
use syn::ItemFn;
use syn::__private::TokenStream2;


pub enum ActionCommand {
    /// without inputs
    Pattern1,
    /// Input with args,
    Pattern2,
    /// Input with WebviewEntity(entity)
    Pattern3,
    /// Input with Webview entity and args.
    Pattern4,
}

pub enum AsyncCommand {
    /// Without inputs
    Pattern1,
    /// With args
    Pattern2,
    /// With Webview entity,
    Pattern3,
    /// With Reactor Task
    Pattern4,
    /// With args and Webview entity
    Pattern5,
    /// With args, Webview Entity and Reactor Task
    Pattern6,
    /// With Webview entity and Reactor Task
    Pattern7,
    /// With args and Reactor task
    Pattern8,
}

pub fn expand_call_fn(
    f: &ItemFn,
    is_internal: bool,
) -> TokenStream2 {
    if f.sig.asyncness.is_some() {
        expand_async_command(f, is_internal)
    } else {
        expand_action_command(f, is_internal)
    }
}

