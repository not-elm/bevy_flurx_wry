use crate::base_module;
use crate::command::Input;
use proc_macro2::TokenStream;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{FnArg, ItemFn, Type};

pub fn expand_action_command(
    f: &ItemFn,
) -> TokenStream2 {
    let fn_ident = &f.sig.ident;
    let module_name = base_module();
    let inputs = parse_action_command_inputs(f, &module_name);
    _expand_action_command(&module_name, quote! { #fn_ident(#(#inputs,)*) })
}

fn _expand_action_command(
    module_name: &TokenStream2,
    f: TokenStream2,
) -> TokenStream2 {
    quote! {
        commands.spawn(bevy_flurx::prelude::Reactor::schedule(move |task| async move{
            use bevy_flurx::prelude::{Map, Pipe};
            task.will(bevy::prelude::Update, #f
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

fn parse_action_command_inputs(f: &ItemFn, module_name: &TokenStream) -> Vec<TokenStream> {
    let mut inputs = Vec::with_capacity(2);
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
            "In" => inputs.push(Input::In.to_token(module_name)),
            "WebviewEntity" => inputs.push(Input::WebviewEntity.to_token(module_name)),
            _ => continue,
        }
    }
    inputs
}