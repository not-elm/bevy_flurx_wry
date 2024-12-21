use crate::base_module;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{FnArg, ItemFn, Type};

enum AsyncCommand {
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
    /// With args and Reactor task
    Pattern6,
    /// With Webview entity and Reactor Task
    Pattern7,
    /// With args, Webview Entity and Reactor Task
    Pattern8,
}

pub fn expand_async_command(
    f: &ItemFn,
    is_internal: bool,
) -> TokenStream2 {
    let fn_ident = &f.sig.ident;
    let module_name = base_module(is_internal);
    let webview_entity = quote! {
        #module_name  WebviewEntity(ipc_cmd.entity)
    };
    match check_async_command_type(f) {
        AsyncCommand::Pattern1 => {
            expand_call(is_internal, &module_name, quote! { #fn_ident().await; })
        }
        AsyncCommand::Pattern2 => {
            expand_call(is_internal, &module_name, quote! { #fn_ident(ipc_cmd.payload.deserialize_args()).await; })
        }
        AsyncCommand::Pattern3 => {
            expand_call(is_internal, &module_name, quote! { #fn_ident(#webview_entity).await; })
        }
        AsyncCommand::Pattern4 => {
            expand_call(is_internal, &module_name, quote! { #fn_ident(task.clone()).await; })
        }
        AsyncCommand::Pattern5 => {
            expand_call(is_internal, &module_name, quote! { #fn_ident(ipc_cmd.payload.deserialize_args(), #webview_entity).await; })
        }
        AsyncCommand::Pattern6 => {
            expand_call(is_internal, &module_name, quote! { #fn_ident(ipc_cmd.payload.deserialize_args(), task.clone()).await; })
        }
        AsyncCommand::Pattern7 => {
            expand_call(is_internal, &module_name, quote! { #fn_ident(#webview_entity, task.clone()).await; })
        }
        AsyncCommand::Pattern8 => {
            expand_call(is_internal, &module_name, quote! { #fn_ident(ipc_cmd.payload.deserialize_args(), #webview_entity, task.clone()).await; })
        }
    }
}

fn expand_call(is_internal: bool, module_name: &TokenStream2, f: TokenStream2) -> TokenStream2 {
    let update_label = if is_internal {
        quote! { bevy_app::prelude::Update }
    } else {
        quote! { bevy::prelude::Update}
    };
    quote! {
        commands.spawn(bevy_flurx::prelude::Reactor::schedule(move |task| async move{
            let output = #f
            task.will(#update_label, bevy_flurx::prelude::once::event::send().with(#module_name IpcResolveEvent{
                    resolve_id: ipc_cmd.payload.resolve_id,
                    entity: ipc_cmd.entity,
                    output: #module_name to_string(output),
                })).await;
        }));
    }
}

fn check_async_command_type(f: &ItemFn) -> AsyncCommand {
    let mut has_in = false;
    let mut has_webview_entity = false;
    let mut has_task = false;
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
            "ReactorTask" => has_task = true,
            _ => continue,
        }
    }
    match (has_in, has_webview_entity, has_task) {
        (false, false, false) => AsyncCommand::Pattern1,
        (true, false, false) => AsyncCommand::Pattern2,
        (false, true, false) => AsyncCommand::Pattern3,
        (false, false, true) => AsyncCommand::Pattern4,
        (true, true, false) => AsyncCommand::Pattern5,
        (true, false, true) => AsyncCommand::Pattern6,
        (false, true, true) => AsyncCommand::Pattern7,
        (true, true, true) => AsyncCommand::Pattern8,
    }
}