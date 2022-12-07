use proc_macro::TokenStream;
use syn::{self, parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};
use quote::quote;


#[proc_macro_derive(WhoAmI)]
pub fn who_am_i(tokens: TokenStream) -> TokenStream {
    // convert the input tokens into an ast, specially from a derive
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
    panic!("My struct name is: <{}>", ast.ident.to_string());

    TokenStream::new()
}

/// REFERENCE: [LogRocket](https://blog.logrocket.com/procedural-macros-in-rust/)
#[proc_macro_derive(Describe)]
pub fn describe(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let description = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let idents = named.iter().map(|f| &f.ident);
                format!(
                    "a struct with named fields: {}",
                    quote! {#(#idents),*}
                )
            }
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let num_fields = unnamed.iter().count();
                format!("a struct with unnamed fields: #{}", num_fields)
            }
            syn::Fields::Unit => String::from("a unit struct")
        }
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let vs = variants.iter().map(|v| &v.ident);
            format!("an enum with these variants: {}",
                quote! {#(#vs),*}
            )
        }
        syn::Data::Union(DataUnion { fields: FieldsNamed { named, .. }, ..}) => {
                let idents = named.iter().map(|f| &f.ident);
                format!("a union with these named fields: {}",
                    quote! {#(#idents),*}
                )
        }
    };
    let output = quote! {
        impl #ident {
            fn describe() {
                println!("{} is {}", stringify!(#ident), #description);
            }
        }
    };
    output.into()
}
