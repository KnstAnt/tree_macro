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

pub trait NewMacro {
    fn new();
}

fn impl_new_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl NewMacro for #name {
            fn new_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(NewMacro)]
pub fn new_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_new_macro(&ast)
}