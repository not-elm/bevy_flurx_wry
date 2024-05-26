//! This crate provides macros to support `bevy_flurx_ipc`.

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::__private::TokenStream2;
use syn::ItemFn;


/// Convert the function to `ipc-command`.
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
///
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
/// It is more advanced than the `action command`.
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
///     task.will(Update, once::run(move ||{
///         println!("{entity:?} {message}");
///     })).await;
/// }
/// ```
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
        #[allow(missing_docs)]
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

