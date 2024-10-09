extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse::Parser, parse_macro_input, DeriveInput, FieldsNamed, Type};
use convert_case::{Case, Casing};

extern crate quote;
extern crate syn;


#[proc_macro_attribute]
pub fn add_field(_args: TokenStream, input: TokenStream) -> TokenStream  {
    let mut ast = parse_macro_input!(input as DeriveInput);
    if let syn::Data::Struct(ref mut struct_data) = &mut ast.data {
        if let syn::Fields::Named(fields) = &mut struct_data.fields {
            let mut new_fields = Vec::new();
            for f in &fields.named {
                let token = quote! { pub stringify!(f.ty).to_case(Case::Snake) : #f.ty };
               // if let Type::Infer(type_path) = f.ty.clone() {
                let field = syn::Field::parse_named.parse2(token).unwrap();
                    new_fields.push(field);
                    //new_fields.push(syn::Field::parse_named.parse2( quote!{ pub type_path.clone().into_token_stream().to_string().to_case(Case::Snake): type_path } ).unwrap());
              //  }
            }
            dbg!(&new_fields);
            new_fields.into_iter().for_each(|f| fields.named.push(f));
        } 

    /*    if let syn::Fields::Named(fields) = &mut struct_data.fields {
            fields.named.push(syn::Field::parse_named.parse2(quote! { pub a: String }).unwrap());
        }         
    */

     /*   if let syn::Fields::Named(fields) = &mut struct_data.fields {
            let mut new_fields = Vec::new();
            for f in &fields.named {
                let token = quote! { pub f.ty.clone().into_token_stream().to_string().to_case(Case::Snake): f.ty };
               // if let Type::Infer(type_path) = f.ty.clone() {
                    new_fields.push(syn::Field::parse_named.parse2(token).unwrap());
                    //new_fields.push(syn::Field::parse_named.parse2( quote!{ pub type_path.clone().into_token_stream().to_string().to_case(Case::Snake): type_path } ).unwrap());
              //  }
            }

            new_fields.into_iter().for_each(|f| fields.named.push(f));
        }
*/
   /*         fields.unnamed.into_iter()
            .map(|f| { 
                if let Type::Verbatim(type_path) = f.ty {
                    syn::Field::parse_named.parse2( quote!{ pub type_path.clone().into_token_stream().to_string().to_case(Case::Snake): ty } ).unwrap().into()
                } else {
                    f
                }
            } ).collect::<syn::Fields>();
    */
        

        return quote! {
            #ast
        }.into();
    }

    panic!("`add_field` has to be used with structs ");
}



/*extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, FieldsNamed, Type};

extern crate quote;
extern crate syn;

#[proc_macro_attribute]
pub fn add_field(_args: TokenStream, input: TokenStream) -> TokenStream  {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {   
        
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    for f in fields.named {
                        f =
                    }
                    
                    fields.named.in = fields
                        .named
                        //.push(syn::Field::parse_named.parse2(quote! { pub a: String })
                        .into_iter()
                        .map(|v| {
                            v.
                        } ).collect();
                }   
                _ => {
                    ()
                }
            }              
            
            return quote! {
                #ast
            }.into();
        }
        _ => panic!("`add_field` has to be used with structs "),
    }
}

#[proc_macro_derive(DoubleF64)]
pub fn double_f64(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let mut func_stream = TokenStream2::default();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let fields = named.iter().map(|f| &f.ident);
            let ftypes = named.iter().map(|f| &f.ty);

            for (field, ftype) in fields.into_iter().zip(ftypes.into_iter()) {
                match ftype {
                    Type::Verbatim(type_path) => type_path.clone().into_token_stream().to_string()
              /*      Type::Path(type_path)
                        if type_path.clone().into_token_stream().to_string() == "f64" =>
                    {
                        let fname = format_ident!("double_{}", field.clone().unwrap());
                        func_stream.extend::<TokenStream2>(
                            quote! { fn #fname(&self) -> f64 { self.#field * 2.0 } },
                        );
                    }*/
                    _ => {}
                };
            }
        }
    };

    let output = quote! {
        impl #ident {
            #func_stream
        }
    };

    output.into()
}


#[proc_macro_attribute]
pub fn add_field(_args: TokenStream, input: TokenStream) -> TokenStream  {
    let mut ast = parse_macro_input!(input as DeriveInput);
    match &mut ast.data {
        syn::Data::Struct(ref mut struct_data) => {   
        
            match &mut struct_data.fields {
                syn::Fields::Named(fields) => {
                    for f in fields.named {
                        f =
                    }

                    fields.named.in = fields
                        .named
                        //.push(syn::Field::parse_named.parse2(quote! { pub a: String })
                        .into_iter()
                        .map(|v| {
                            v.
                        } ).collect();
                }   
                _ => {
                    ()
                }
            }              
            
            return quote! {
                #ast
            }.into();
        }
        _ => panic!("`add_field` has to be used with structs "),
    }
}


*/