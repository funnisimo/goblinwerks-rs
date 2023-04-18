use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Type};
use syn::{FnArg, ItemFn};

pub fn make_system_fn(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_ident = input_fn.sig.ident.clone();
    let sys_fn_ident = format_ident!("{}_system", input_fn.sig.ident);

    println!("=========== {} =============", sys_fn_ident);

    let mut arg_types: Vec<Box<Type>> = Vec::new();
    for arg in input_fn.sig.inputs.iter() {
        match arg {
            FnArg::Typed(t) => {
                println!("Typed arg type = {:?}", t.ty);
                arg_types.push(t.ty.clone());
            }
            FnArg::Receiver(r) => println!("Receiver Arg = {:?}", r),
        }
    }

    let i = (0..arg_types.len()).map(syn::Index::from);

    quote! {
        #input_fn

        fn #sys_fn_ident(ecs: &Ecs) {
            let data = <(#(#arg_types),*,)>::fetch(ecs);
            #fn_ident(#(data.#i ),*);
        }
    }
    .into()
}
