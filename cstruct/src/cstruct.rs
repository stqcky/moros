use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Attribute, DeriveInput, Field, Fields};

pub fn cstruct_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields: Fields = match input.data {
        syn::Data::Struct(data) => data.fields,
        syn::Data::Union(data) => data.fields.into(),
        syn::Data::Enum(_) => panic!("#[derive(C)] cannot be applied to an enum"),
    };

    let getters = generate_getters(fields);

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #(#getters)*
        }
    }
    .into()
}

fn generate_getters(fields: Fields) -> Vec<TokenStream2> {
    fields
        .iter()
        .filter_map(|field| {
            let attrs = &field.attrs;

            if has_tag(attrs, "str") {
                Some(generate_str_getter(&field))
            } else if has_tag(attrs, "ptr") {
                Some(generate_ptr_getter(&field))
            } else {
                None
            }
        })
        .collect()
}

fn generate_str_getter(field: &Field) -> TokenStream2 {
    let field_name = field.clone().ident.unwrap();

    let array_suffix = match &field.ty {
        syn::Type::Array(_) => quote! {
            .as_ptr()
        },
        _ => quote! {},
    };

    quote! {
        pub fn #field_name(&self) -> Cow<'_, str> {
            unsafe { std::ffi::CStr::from_ptr((self.#field_name)#array_suffix).to_string_lossy() }
        }
    }
}

fn generate_ptr_getter(field: &Field) -> TokenStream2 {
    let field_name = field.clone().ident.unwrap();

    let field_type = match &field.ty {
        syn::Type::Ptr(ptr) => &ptr.elem,
        _ => panic!("#[ptr] field should be a raw pointer"),
    };

    quote! {
        pub fn #field_name(&self) -> Option<&#field_type> {
            unsafe { self.#field_name.as_ref() }
        }
    }
}

fn has_tag(atrs: &Vec<Attribute>, tag: &str) -> bool {
    atrs.iter()
        .find(|attr| attr.meta.path().is_ident(tag))
        .is_some()
}
