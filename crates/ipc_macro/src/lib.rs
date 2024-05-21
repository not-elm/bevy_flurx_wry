use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, input: TokenStream) -> TokenStream {
    parse_command(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn parse_command(input: TokenStream) -> syn::Result<TokenStream2> {
    let mut f = syn::parse::<ItemFn>(input)?;
    let fn_ident = f.sig.ident.clone();
    let fn_name = fn_ident.to_string();
    f.sig.ident = Ident::new("internal", Span::call_site());

    Ok(quote! {
        #[allow(non_snake_case)]
        pub fn #fn_ident() -> bevy_flurx_ipc::prelude::IpcHandler{
            #f
            use bevy_flurx_ipc::prelude::Functor;
            bevy_flurx_ipc::prelude::IpcHandler::new(#fn_name, ||{
                internal
            })
        }
    })
}


