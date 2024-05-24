use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn command(attr: TokenStream, input: TokenStream) -> TokenStream {
    let custom_id = parse_custom_id(attr);
    parse_command(input, custom_id)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn parse_command(input: TokenStream, custom_id: Option<String>) -> syn::Result<TokenStream2> {
    let mut f = syn::parse::<ItemFn>(input)?;
    let fn_ident = f.sig.ident.clone();
    let ipc_id = custom_id.unwrap_or(fn_ident.to_string());
    f.sig.ident = Ident::new("internal", Span::call_site());

    Ok(quote! {
        #[allow(non_snake_case)]
        pub fn #fn_ident() -> bevy_flurx_ipc::prelude::IpcHandler{
            #f
            use bevy_flurx_ipc::prelude::Functor;
            bevy_flurx_ipc::prelude::IpcHandler::new(#ipc_id, ||{
                internal
            })
        }
    })
}

fn parse_custom_id(attr: TokenStream) -> Option<String> {
    let errors = deluxe::Errors::new();
    let Args { id } = deluxe::parse_optional(attr, &errors);
    id
}

#[derive(Default, deluxe::ParseMetaItem, deluxe::ExtractAttributes)]
struct Args {
    id: Option<String>,
}

