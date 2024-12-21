use crate::base_module;
use crate::command::Input;
use proc_macro2::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{FnArg, ItemFn, Type};

pub fn expand_async_command(
    f: &ItemFn,
    is_internal: bool,
) -> TokenStream2 {
    let fn_ident = &f.sig.ident;
    let module_name = base_module(is_internal);
    let inputs = parse_async_command_inputs(f, &module_name);
    expand_call(is_internal, &module_name, quote! { #fn_ident(#(#inputs,)*).await; })
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

fn parse_async_command_inputs(f: &ItemFn, module_name: &TokenStream) -> Vec<TokenStream> {
    let mut args = Vec::with_capacity(3);
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
            "In" => args.push(Input::In.to_token(module_name)),
            "WebviewEntity" => args.push(Input::WebviewEntity.to_token(module_name)),
            "ReactorTask" => args.push(Input::Task.to_token(module_name)),
            _ => continue,
        }
    }
    args
}