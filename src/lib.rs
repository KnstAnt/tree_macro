use proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn var_name(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident).to_string();
    let expanded = quote! {
        {
            println!("{}", #name);
        }
    };
    TokenStream::from(expanded)
}