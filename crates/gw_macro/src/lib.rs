use proc_macro::TokenStream;

mod system;

#[proc_macro_attribute]
pub fn system(args: TokenStream, item: TokenStream) -> TokenStream {
    system::make_system_fn(args, item)
}
