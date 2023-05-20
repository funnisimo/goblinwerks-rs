#![recursion_limit = "256"]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::ParseStream,
    parse_macro_input, // Path,
    parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    ConstParam,
    // parenthesized,
    // parse::{Parse, ParseStream, Result},
    DeriveInput,
    GenericParam,
    Ident,
    Index,
    Meta,
    Token,
    TypeParam,
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

    match storage {
        Some(Ok(storage)) => {
            quote! {
                impl #impl_generics Component for #name #ty_generics #where_clause {
                    type Storage = #storage<Self>;
                }
            }
        }
        _ => {
            quote! {
                impl #impl_generics Component for #name #ty_generics #where_clause {
                    type Storage = gw_ecs::storage::DenseVecStorage<Self>;
                }
            }
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

#[derive(Default)]
struct SystemParamFieldAttributes {
    pub ignore: bool,
}

static SYSTEM_PARAM_ATTRIBUTE_NAME: &str = "system_param";

/// Implement `SystemParam` to use a struct as a parameter in a system
#[proc_macro_derive(SystemParam, attributes(system_param))]
pub fn derive_system_param(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let syn::Data::Struct(syn::DataStruct { fields: field_definitions, .. }) = ast.data else {
        return syn::Error::new(ast.span(), "Invalid `SystemParam` type: expected a `struct`")
            .into_compile_error()
            .into();
    };
    // let path: syn::Path = bevy_ecs_path();

    let field_attributes = field_definitions
        .iter()
        .map(|field| {
            (
                field,
                field
                    .attrs
                    .iter()
                    .find(|a| {
                        *a.path().get_ident().as_ref().unwrap() == SYSTEM_PARAM_ATTRIBUTE_NAME
                    })
                    .map_or_else(SystemParamFieldAttributes::default, |a| {
                        syn::custom_keyword!(ignore);
                        let mut attributes = SystemParamFieldAttributes::default();
                        a.parse_args_with(|input: ParseStream| {
                            if input.parse::<Option<ignore>>()?.is_some() {
                                attributes.ignore = true;
                            }
                            Ok(())
                        })
                        .expect("Invalid 'system_param' attribute format.");

                        attributes
                    }),
            )
        })
        .collect::<Vec<_>>();

    let mut field_locals = Vec::new();
    let mut fields = Vec::new();
    let mut field_types = Vec::new();
    let mut ignored_fields = Vec::new();
    let mut ignored_field_types = Vec::new();
    for (i, (field, attrs)) in field_attributes.iter().enumerate() {
        if attrs.ignore {
            ignored_fields.push(field.ident.as_ref().unwrap());
            ignored_field_types.push(&field.ty);
        } else {
            field_locals.push(format_ident!("f{i}"));
            let i = Index::from(i);
            fields.push(
                field
                    .ident
                    .as_ref()
                    .map(|f| quote! { #f })
                    .unwrap_or_else(|| quote! { #i }),
            );
            field_types.push(&field.ty);
        }
    }

    let generics = ast.generics;

    // Emit an error if there's any unrecognized lifetime names.
    for lt in generics.lifetimes() {
        let ident = &lt.lifetime.ident;
        let w = format_ident!("w");
        let s = format_ident!("s");
        if ident != &w && ident != &s {
            return syn::Error::new_spanned(
                lt,
                r#"invalid lifetime name: expected `'w` or `'s`
 'w -- refers to data stored in the World.
 's -- refers to data stored in the SystemParam's state.'"#,
            )
            .into_compile_error()
            .into();
        }
    }

    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let lifetimeless_generics: Vec<_> = generics
        .params
        .iter()
        .filter(|g| !matches!(g, GenericParam::Lifetime(_)))
        .collect();

    let shadowed_lifetimes: Vec<_> = generics.lifetimes().map(|_| quote!('_)).collect();

    let mut punctuated_generics = Punctuated::<_, Token![,]>::new();
    punctuated_generics.extend(lifetimeless_generics.iter().map(|g| match g {
        GenericParam::Type(g) => GenericParam::Type(TypeParam {
            default: None,
            ..g.clone()
        }),
        GenericParam::Const(g) => GenericParam::Const(ConstParam {
            default: None,
            ..g.clone()
        }),
        _ => unreachable!(),
    }));

    let mut punctuated_generic_idents = Punctuated::<_, Token![,]>::new();
    punctuated_generic_idents.extend(lifetimeless_generics.iter().map(|g| match g {
        GenericParam::Type(g) => &g.ident,
        GenericParam::Const(g) => &g.ident,
        _ => unreachable!(),
    }));

    let punctuated_generics_no_bounds: Punctuated<_, Token![,]> = lifetimeless_generics
        .iter()
        .map(|&g| match g.clone() {
            GenericParam::Type(mut g) => {
                g.bounds.clear();
                GenericParam::Type(g)
            }
            g => g,
        })
        .collect();

    let mut tuple_types: Vec<_> = field_types.iter().map(|x| quote! { #x }).collect();
    let mut tuple_patterns: Vec<_> = field_locals.iter().map(|x| quote! { #x }).collect();

    tuple_types.extend(
        ignored_field_types
            .iter()
            .map(|ty| parse_quote!(::std::marker::PhantomData::<#ty>)),
    );
    tuple_patterns.extend(ignored_field_types.iter().map(|_| parse_quote!(_)));

    // If the number of fields exceeds the 16-parameter limit,
    // fold the fields into tuples of tuples until we are below the limit.
    const LIMIT: usize = 16;
    while tuple_types.len() > LIMIT {
        let end = Vec::from_iter(tuple_types.drain(..LIMIT));
        tuple_types.push(parse_quote!( (#(#end,)*) ));

        let end = Vec::from_iter(tuple_patterns.drain(..LIMIT));
        tuple_patterns.push(parse_quote!( (#(#end,)*) ));
    }

    // Create a where clause for the `ReadOnlySystemParam` impl.
    // Ensure that each field implements `ReadOnlySystemParam`.
    let mut read_only_generics = generics.clone();
    let read_only_where_clause = read_only_generics.make_where_clause();
    for field_type in &field_types {
        read_only_where_clause
            .predicates
            .push(syn::parse_quote!(#field_type: gw_ecs::bevy::ReadOnlySystemParam));
    }

    let fields_alias = format_ident!("__StructFieldsAlias");

    let struct_name = &ast.ident;
    let state_struct_visibility = &ast.vis;

    TokenStream::from(quote! {
        // We define the FetchState struct in an anonymous scope to avoid polluting the user namespace.
        // The struct can still be accessed via SystemParam::State, e.g. EventReaderState can be accessed via
        // <EventReader<'static, 'static, T> as SystemParam>::State
        const _: () = {
            // Allows rebinding the lifetimes of each field type.
            type #fields_alias <'w, 's, #punctuated_generics_no_bounds> = (#(#tuple_types,)*);

            #[doc(hidden)]
            #state_struct_visibility struct FetchState <#(#lifetimeless_generics,)*>
            #where_clause {
                state: <#fields_alias::<'static, 'static, #punctuated_generic_idents> as gw_ecs::bevy::SystemParam>::State,
            }

            unsafe impl<#punctuated_generics> gw_ecs::bevy::SystemParam for
                #struct_name <#(#shadowed_lifetimes,)* #punctuated_generic_idents> #where_clause
            {
                type State = FetchState<#punctuated_generic_idents>;
                type Item<'w, 's> = #struct_name #ty_generics;

                fn init_state(world: &mut gw_ecs::world::World, system_meta: &mut gw_ecs::bevy::SystemMeta) -> Self::State {
                    FetchState {
                        state: <#fields_alias::<'_, '_, #punctuated_generic_idents> as gw_ecs::bevy::SystemParam>::init_state(world, system_meta),
                    }
                }

                // fn new_archetype(state: &mut Self::State, archetype: &gw_ecs::archetype::Archetype, system_meta: &mut gw_ecs::bevy::SystemMeta) {
                //     <#fields_alias::<'_, '_, #punctuated_generic_idents> as gw_ecs::bevy::SystemParam>::new_archetype(&mut state.state, archetype, system_meta)
                // }

                fn apply(state: &mut Self::State, system_meta: &gw_ecs::bevy::SystemMeta, world: &mut gw_ecs::world::World) {
                    <#fields_alias::<'_, '_, #punctuated_generic_idents> as gw_ecs::bevy::SystemParam>::apply(&mut state.state, system_meta, world);
                }

                unsafe fn get_param<'w, 's>(
                    state: &'s mut Self::State,
                    system_meta: &gw_ecs::bevy::SystemMeta,
                    world: &'w gw_ecs::world::World,
                    change_tick: u32,
                ) -> Self::Item<'w, 's> {
                    let (#(#tuple_patterns,)*) = <
                        (#(#tuple_types,)*) as gw_ecs::bevy::SystemParam
                    >::get_param(&mut state.state, system_meta, world, change_tick);
                    #struct_name {
                        #(#fields: #field_locals,)*
                        #(#ignored_fields: std::default::Default::default(),)*
                    }
                }
            }

            // Safety: Each field is `ReadOnlySystemParam`, so this can only read from the `World`
            unsafe impl<'w, 's, #punctuated_generics> gw_ecs::bevy::ReadOnlySystemParam for #struct_name #ty_generics #read_only_where_clause {}
        };
    })
}
