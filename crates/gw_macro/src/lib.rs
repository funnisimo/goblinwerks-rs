#![recursion_limit = "256"]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    // parenthesized,
    // parse::{Parse, ParseStream, Result},
    DeriveInput,
    Ident,
    Meta, // Path,
};

// mod saveload;
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

/// Custom derive macro for the `Component` trait.
///
/// ## Example
///
/// ```rust,ignore
/// use specs::storage::VecStorage;
///
/// #[derive(Component, Debug)]
/// #[storage(VecStorage)] // This line is optional, defaults to `DenseVecStorage`
/// struct Pos(f32, f32, f32);
/// ```
#[proc_macro_derive(Component, attributes(storage))]
pub fn component(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = impl_component(&ast);
    gen.into()
}

// struct StorageAttribute {
//     storage: Path,
// }

// impl Parse for StorageAttribute {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let content;
//         let _parenthesized_token = parenthesized!(content in input);

//         Ok(StorageAttribute {
//             storage: content.parse()?,
//         })
//     }
// }

fn impl_component(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let storage = ast
        .attrs
        .iter()
        .find(|attr| attr.path().segments[0].ident == "storage")
        .map(|attr| {
            match attr.meta.clone() {
                Meta::List(names) => syn::parse2::<Ident>(names.tokens.clone()),
                Meta::Path(_path) => Err(syn::Error::new_spanned(attr, "Invalid storage value")),
                Meta::NameValue(_name_value) => {
                    Err(syn::Error::new_spanned(attr, "Invalid storage value"))
                }
            }
            // syn::parse2::<StorageAttribute>(attr.meta.require_list().unwrap().tokens.clone())
            //     .unwrap()
            //     .storage
        });

    let storage = match storage {
        None => format_ident!("DenseVecStorage"),
        Some(Ok(ident)) => ident,
        Some(Err(_)) => format_ident!("DenseVecStorage"),
    };

    quote! {
        impl #impl_generics Component for #name #ty_generics #where_clause {
            type Storage = #storage<Self>;
        }
    }
}

/*
/// Custom derive macro for the `ConvertSaveload` trait.
///
/// Requires `Entity`, `ConvertSaveload`, `Marker` to be in a scope
///
/// ## Example
///
/// ```rust,ignore
/// use specs::{Entity, saveload::{ConvertSaveload, Marker}};
///
/// #[derive(ConvertSaveload)]
/// struct Target(Entity);
/// ```
#[proc_macro_derive(
    ConvertSaveload,
    attributes(convert_save_load_attr, convert_save_load_skip_convert)
)]
pub fn saveload(input: TokenStream) -> TokenStream {
    use saveload::impl_saveload;
    let mut ast = syn::parse(input).unwrap();

    let gen = impl_saveload(&mut ast);
    gen.into()
}
*/
