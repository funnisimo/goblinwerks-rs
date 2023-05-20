// This is an adapted version of shred-derive/src/mod.rs
// It is adapted to handle the changes made in gw_specs to convert shred/specs to using globals and multiple worlds.

use quote::quote;
use syn::{
    parse_quote, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput, Field,
    Fields, FieldsNamed, FieldsUnnamed, Ident, Lifetime, Type, WhereClause, WherePredicate,
};

pub fn impl_system_data(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let mut generics = ast.generics.clone();

    let (fetch_return, tys) = gen_from_body(&ast.data, name);
    let tys = &tys;
    // Assumes that the first lifetime is the fetch lt
    let def_fetch_lt = ast
        .generics
        .lifetimes()
        .next()
        .expect("There has to be at least one lifetime");
    let impl_fetch_lt = &def_fetch_lt.lifetime;

    {
        let where_clause = generics.make_where_clause();
        constrain_system_data_types(where_clause, impl_fetch_lt, tys);
    }
    // Reads and writes are taken from the same types,
    // but need to be cloned before.

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics
            SystemData< #impl_fetch_lt >
            for #name #ty_generics #where_clause
        {
            fn setup(world: &mut gw_ecs::World) {
                #(
                    <#tys as SystemData> :: setup(world);
                )*
            }

            fn fetch(world: &'a gw_ecs::World) -> Self {
                #fetch_return
            }

            fn reads() -> std::collections::HashSet<gw_ecs::ResourceId> {
                let mut r = std::collections::HashSet::new();

                #( {
                        for i in <#tys as SystemData>::reads().into_iter() {
                            r.insert(i);
                        }
                    } )*

                r
            }

            fn writes() -> std::collections::HashSet<gw_ecs::ResourceId> {
                let mut r = std::collections::HashSet::new();

                #( {
                        for i in <#tys as SystemData>::writes().into_iter() {
                            r.insert(i);
                        }
                    } )*

                r
            }
        }
    }
}

fn collect_field_types(fields: &Punctuated<Field, Comma>) -> Vec<Type> {
    fields.iter().map(|x| x.ty.clone()).collect()
}

fn gen_identifiers(fields: &Punctuated<Field, Comma>) -> Vec<Ident> {
    fields.iter().map(|x| x.ident.clone().unwrap()).collect()
}

/// Adds a `SystemData<'lt>` bound on each of the system data types.
fn constrain_system_data_types(clause: &mut WhereClause, fetch_lt: &Lifetime, tys: &[Type]) {
    for ty in tys.iter() {
        let where_predicate: WherePredicate = parse_quote!(#ty : SystemData< #fetch_lt >);
        clause.predicates.push(where_predicate);
    }
}

fn gen_from_body(ast: &Data, name: &Ident) -> (proc_macro2::TokenStream, Vec<Type>) {
    enum DataType {
        Struct,
        Tuple,
    }

    let (body, fields) = match *ast {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named: ref x, .. }),
            ..
        }) => (DataType::Struct, x),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { unnamed: ref x, .. }),
            ..
        }) => (DataType::Tuple, x),
        _ => panic!("Enums are not supported"),
    };

    let tys = collect_field_types(fields);

    let fetch_return = match body {
        DataType::Struct => {
            let identifiers = gen_identifiers(fields);

            quote! {
                #name {
                    #( #identifiers: SystemData::fetch(world) ),*
                }
            }
        }
        DataType::Tuple => {
            let count = tys.len();
            let fetch = vec![quote! { SystemData::fetch(world) }; count];

            quote! {
                #name ( #( #fetch ),* )
            }
        }
    };

    (fetch_return, tys)
}
