mod async_command;
mod action_command;

use crate::command::action_command::expand_action_command;
use crate::command::async_command::expand_async_command;
use syn::ItemFn;
use syn::__private::TokenStream2;






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

