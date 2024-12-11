//! This crate provides macros to support `bevy_flurx_ipc`.

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
/// use bevy_flurx_ipc::command;
/// use bevy_flurx::prelude::*;
/// use bevy_flurx_ipc::prelude::WebviewEntity;
///
/// #[command]
/// fn case1() -> ActionSeed<(), String>{
///     once::run(|| "output is returned to Javascript".to_string())
/// }
///
/// #[command]
/// fn case2(WebviewEntity(entity): WebviewEntity) -> ActionSeed{
///     once::run(move ||{
///         println!("{entity:?}");
///     })
/// }
///
/// #[command]
/// fn case3(In(message): In<String>) -> ActionSeed {
///     once::run(move ||{
///         println!("message from javascript: {message}");
///     })
/// }
///
/// #[command]
/// fn case4(In(message): In<String>, WebviewEntity(entity): WebviewEntity) -> ActionSeed{
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
/// use bevy_flurx_ipc::command;
/// use bevy_flurx::prelude::*;
/// use bevy_flurx_ipc::component::WebviewEntity;
///
/// #[command]
/// async fn case1() -> String{
///     "output is returned to Javascript".to_string()
/// }
///
/// #[command]
/// async fn case2(In(message): In<String>){
///     println!("{message}");
/// }
///
/// #[command]
/// async fn case3(task: ReactiveTask) {
///     task.will(Update, once::run(||{})).await
/// }
///
/// #[command]
/// async fn case4(In(message): In<String>, task: ReactiveTask){
///     task.will(Update, once::run(move ||{
///         println!("{message}");
///     })).await;
/// }
///
/// #[command]
/// async fn case5(In(message): In<String>, WebviewEntity(entity): WebviewEntity, task: ReactiveTask){
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
    let mut f = syn::parse::<ItemFn>(input)?;
    let fn_ident = f.sig.ident.clone();
    let ipc_id = custom_id.unwrap_or(fn_ident.to_string());
    let crate_name = if attribute.is_some_and(|attr| attr.internal.is_present()) {
        "bevy_flurx_ipc"
    } else {
        "bevy_flurx_wry"
    };
    let crate_name = Ident::new(crate_name, Span::call_site());
    f.sig.ident = Ident::new("internal", Span::call_site());
    let visibility = &f.vis;

    Ok(quote! {
        #[allow(missing_docs)]
        #visibility fn #fn_ident() -> #crate_name::prelude::IpcHandler{
            #f
            use #crate_name::prelude::Functor;
            #crate_name::prelude::IpcHandler::new(#ipc_id, ||{
                internal
            })
        }
    })
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

