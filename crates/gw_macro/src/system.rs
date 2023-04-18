use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;
use syn::ItemFn;

pub fn make_system_fn(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let fn_ident = input_fn.sig.ident.clone();
    let sys_fn_ident = format_ident!("{}_system", input_fn.sig.ident);

    println!("=========== {} =============", sys_fn_ident);

    for arg in input_fn.sig.inputs.iter() {
        println!("arg = {:?}", arg);
    }

    quote! {
        #input_fn

        fn #sys_fn_ident(ecs: &Ecs) {
            let data = <(&Ecs,)>::fetch(ecs);
            #fn_ident(data.0);
        }
    }
    .into()
}
