#![recursion_limit = "256"]

use proc_macro::TokenStream;
mod system;

/// Used to `#[derive]` the trait `SystemData`.
///
/// You need to have the following items included in the current scope:
///
/// * `SystemData`
/// * `World`
/// * `ResourceId`
///
/// This macro can either be used directly via `shred-derive`, or by enabling
/// the `shred-derive` feature for another crate (e.g. `shred` or `specs`, which
/// both reexport the macro).
#[proc_macro_derive(SystemData)]
pub fn system_data(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    let gen = system::impl_system_data(&ast);

    gen.into()
}
