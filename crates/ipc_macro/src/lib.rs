//! This crate provides macros to support `bevy_flurx_ipc`.

mod command;

use crate::command::expand_call_fn;
use darling::ast::NestedMeta;
use darling::util::Flag;
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::ItemFn;
use syn::__private::TokenStream2;

/// Convert the function to `ipc-command`.
///
/// ## Parameters
///
/// - `id` to specify ipc-id: if not specified, ipc-id will be the same as the function name.  
///
/// ## Command Patterns
///
/// ### Action Command
///
/// The function that returns [`Action`](bevy_flurx::prelude::Action) or [`ActionSeed`](bevy_flurx::prelude::ActionSeed) 
/// is tentatively called `action command`.
///
/// You can optionally specify arguments from Javascript and [`WebviewEntity`](bevy_flurx_ipc::prelude::WebviewEntity) as arguments.
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_flurx::prelude::*;
/// use bevy_flurx_wry::ipc::command;
/// use bevy_flurx_wry::ipc::component::WebviewEntity;
///
/// #[command]
/// fn pattern1() -> ActionSeed<(), String>{
///     once::run(|| "output is returned to Javascript".to_string())
/// }
///
/// #[command]
/// fn pattern2(WebviewEntity(entity): WebviewEntity) -> ActionSeed{
///     once::run(move ||{
///         println!("{entity:?}");
///     })
/// }
///
/// #[command]
/// fn pattern3(In(message): In<String>) -> ActionSeed {
///     once::run(move ||{
///         println!("message from javascript: {message}");
///     })
/// }
///
/// #[command]
/// fn pattern4(In(message): In<String>, WebviewEntity(entity): WebviewEntity) -> ActionSeed{
///     once::run(move ||{
///         println!("{message} {entity:?}");
///     })
/// }
/// ```
///
/// ### Task Command
///
/// Asynchronous functions that return output to Javascript are called `task action`.
///
/// This allows for more advanced implementations than `action command`, such as conditional branching and repetition.
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_flurx::prelude::*;
/// use bevy_flurx_wry::ipc::command;
/// use bevy_flurx_wry::ipc::component::WebviewEntity;
///
/// #[command]
/// async fn pattern1() -> String{
///     "output is returned to Javascript".to_string()
/// }
///
/// #[command]
/// async fn pattern2(In(message): In<String>){
///     println!("{message}");
/// }
///
/// #[command]
/// async fn pattern3(task: ReactorTask) {
///     task.will(Update, once::run(||{})).await
/// }
///
/// #[command]
/// async fn pattern4(In(message): In<String>, task: ReactorTask){
///     task.will(Update, once::run(move ||{
///         println!("{message}");
///     })).await;
/// }
///
/// #[command]
/// async fn pattern5(In(message): In<String>, WebviewEntity(entity): WebviewEntity, task: ReactorTask){
///     // `task command` also allows you to use repetition.
///     for _ in 0..3{
///         let message = message.clone();
///         task.will(Update, once::run(move ||{
///             println!("{entity:?} {message}");
///         })).await;    
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn command(attr: TokenStream, input: TokenStream) -> TokenStream {
    let attribute = parse_attribute(attr);
    parse_command(input, attribute)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn parse_command(input: TokenStream, attribute: Option<Attribute>) -> syn::Result<TokenStream2> {
    let custom_id = attribute.as_ref().and_then(|attr| attr.id.clone());
    let f = syn::parse::<ItemFn>(input)?;
    let fn_ident = &f.sig.ident.clone();
    let ipc_id = custom_id.unwrap_or(fn_ident.to_string());
    let is_internal = attribute.is_some_and(|attr| attr.internal.is_present());
    let call_fn = expand_call_fn(&f, is_internal);
    let crate_name = if is_internal {
        "bevy_flurx_ipc"
    } else {
        "bevy_flurx_wry"
    };
    let crate_name = Ident::new(crate_name, Span::call_site());
    let fn_ident = &f.sig.ident;
    let visibility = &f.vis;

    Ok(quote! {
        #[allow(missing_docs)]
        #visibility fn #fn_ident() -> #crate_name::prelude::IpcHandler{
            #crate_name::prelude::IpcHandler::new(#ipc_id, |commands, ipc_cmd|{
                #f
                #call_fn
            })
        }
    })
}

fn base_module(is_internal: bool) -> TokenStream2 {
    if is_internal {
        quote! {  bevy_flurx_ipc::prelude:: }
    } else {
        quote! {  bevy_flurx_wry::prelude:: }
    }
}

fn parse_attribute(attr: TokenStream) -> Option<Attribute> {
    let attr_args = NestedMeta::parse_meta_list(attr.into()).ok()?;
    Attribute::from_list(&attr_args).ok()
}

#[derive(Default, FromMeta)]
struct Attribute {
    id: Option<String>,
    internal: Flag,
}

