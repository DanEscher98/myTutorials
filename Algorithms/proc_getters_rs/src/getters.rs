use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Attribute, Data, DataStruct, DeriveInput, Fields, GenericArgument, Ident, Path, PathArguments,
    Type, TypePath, TypeReference,
};

fn option_inner_type(path: &Path) -> Option<&Type> {
    if path.leading_colon.is_some() {
        return None;
    }
    if path.segments.len() != 1 || path.segments[0].ident != "Option" {
        return None;
    }
    let ab = match &path.segments[0].arguments {
        PathArguments::AngleBracketed(ab) => ab,
        _ => return None,
    };
    if ab.args.len() != 1 {
        return None;
    }

    match &ab.args[0] {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

fn match_ref_field(field_name: Ident, field_type: Type) -> (TokenStream, TokenStream) {
    match field_type {
        Type::Reference(
            r @ TypeReference {
                mutability: None, ..
            },
        ) => (quote! { #r }, quote! { self.#field_name }),
        Type::Path(TypePath { path, .. }) if path.is_ident("String") => (
            quote! { &::core::primitive::str },
            quote! { &self.#field_name },
        ),
        Type::Path(ty @ TypePath { .. }) => match option_inner_type(&ty.path) {
            // Option<String> => Option<&str> (.as_deref())
            Some(Type::Path(TypePath { path, .. })) if path.is_ident("String") => (
                quote! { ::std::option::Option<&::core::primitive::str> },
                quote! { self.#field_name.as_deref() },
            ),
            // Option<T> => Option<&T> (.as_ref())
            Some(inner_ty) => (
                quote! { ::std::option::Option<&#inner_ty> },
                quote! { self.#field_name.as_ref() },
            ),
            None => (quote! { &#ty }, quote! { &self.#field_name }),
        },
        ty => (quote! { &#ty }, quote! { &self.#field_name }),
    }
}

/// 1. [ ] get a MetaList
/// 2. [ ] get the nested value, validate there's just ONE
/// 3. [ ] get a NameValue, validate it's called "name"
/// 4. [ ] validate the value is LitStr
fn get_name_attr(attr: &syn::Attribute) -> syn::Result<Option<Ident>> {
    let meta_list = match attr.meta {
        syn::Meta::List(list) => list,
        _ => {
            return Err(syn::Error::new_spanned(
                attr.meta,
                "expected a list-style attribute",
            ))
        }
    };

    let nested = match meta_list.nested.len() {
        // `#[getter()]` without any argument is a no-op
        0 => return Ok(None),
        1 => &meta_list.nested[0],
        _ => {
            return Err(syn::Error::new_spanned(
                meta_list.nested,
                "currently only a single getter attribute is supported",
            ));
        }
    };

    let name_value = match nested {
        syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => nv,
        _ => {
            return Err(syn::Error::new_spanned(
                nested,
                "expected `name = \"<value\"`",
            ))
        }
    };

    if !name_value.path.is_ident("name") {
        return Err(syn::Error::new_spanned(
            &name_value.path,
            "unsupported getter attribute, expected `name`",
        ));
    }

    match &name_value.lit {
        syn::Lit::Str(s) => syn::parse_str(&s.value()).map_err(|e| syn::Error::new_spanned(s, e)),
        lit => Err(syn::Error::new_spanned(lit, "expected string literal")),
    }
}

pub fn expand_getters(input: DeriveInput) -> TokenStream {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields."),
    };
    let st_name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let getters = fields.into_iter().map(|field| {
        let field_name = field.ident.unwrap();
        let (return_type, body) = match_ref_field(field_name.clone(), field.ty);

        quote! {
            pub fn #field_name(&self) -> #return_type {
                #body
            }
        }
    });

    quote! {
        #[automatically_derived]
        impl #impl_generics #st_name #ty_generics #where_clause {
            #(#getters)*
        }
    }
}
