use crate::base_module;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{FnArg, ItemFn, Type};

enum ActionCommand {
    /// without inputs
    Pattern1,
    /// With the args,
    Pattern2,
    /// With WebviewEntity(entity)
    Pattern3,
    /// With the args and Webview entity 
    Pattern4,
}

pub fn expand_action_command(
    f: &ItemFn,
    is_internal: bool,
) -> TokenStream2 {
    let fn_ident = &f.sig.ident;
    let module_name = base_module(is_internal);
    let webview_entity = quote! {
        #module_name  WebviewEntity(ipc_cmd.entity)
    };
    let args = quote! { ipc_cmd.payload.deserialize_args()};
    match check_action_command_type(f) {
        ActionCommand::Pattern1 => {
            _expand_action_command(is_internal, &module_name, quote! { #fn_ident() })
        }
        ActionCommand::Pattern2 => {
            _expand_action_command(is_internal, &module_name, quote! { #fn_ident(#args) })
        }
        ActionCommand::Pattern3 => {
            _expand_action_command(is_internal, &module_name, quote! { #fn_ident(#webview_entity) })
        }
        ActionCommand::Pattern4 => {
            _expand_action_command(is_internal, &module_name, quote! { #fn_ident(#args, #webview_entity) })
        }
    }
}

fn _expand_action_command(
    is_internal: bool,
    module_name: &TokenStream2,
    f: TokenStream2,
) -> TokenStream2 {
    let update_label = if is_internal {
        quote! { bevy_app::prelude::Update }
    } else {
        quote! { bevy::prelude::Update}
    };

    quote! {
        commands.spawn(bevy_flurx::prelude::Reactor::schedule(move |task| async move{
            use bevy_flurx::prelude::{Map, Pipe};
            task.will(#update_label, #f
                .map(move |output| #module_name IpcResolveEvent{
                    resolve_id: ipc_cmd.payload.resolve_id,
                    entity: ipc_cmd.entity,
                    output: #module_name to_string(output),
                })
                .pipe(bevy_flurx::prelude::once::event::send())
            ).await;
        }));
    }
}

fn check_action_command_type(f: &ItemFn) -> ActionCommand {
    let mut has_in = false;
    let mut has_webview_entity = false;
    for arg in f
        .sig
        .inputs
        .iter() {
        let FnArg::Typed(pat_type) = arg else {
            continue;
        };
        let Type::Path(path) = &*pat_type.ty  else {
            continue;
        };
        let Some(last_segment) = path.path.segments.last() else {
            continue;
        };
        match last_segment.ident.to_string().as_str() {
            "In" => has_in = true,
            "WebviewEntity" => has_webview_entity = true,
            _ => continue,
        }
    }
    match (has_in, has_webview_entity) {
        (false, false) => ActionCommand::Pattern1,
        (true, false) => ActionCommand::Pattern2,
        (false, true) => ActionCommand::Pattern3,
        (true, true) => ActionCommand::Pattern4,
    }
}