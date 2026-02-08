use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    let sql_str = input.to_string();

    let generated = quote! {
        {
            println!("SQL statement captured: {}", #sql_str);
            #sql_str
        }
    };
    generated.into()
}
