use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream { 
    let attr_str = attr.to_string();
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let block = &input_fn.block;

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;

    let generated = quote! {
        #vis #sig {
            println!("Route macro attached: {} -> {}", #attr_str, stringify!(#fn_name));
            #block
        }
    };
    generated.into()
}