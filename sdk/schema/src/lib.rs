use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, DeriveInput, Field, Fields,
    GenericArgument, LitStr, Meta, PathArguments, Token, Type, TypePath,
};

#[proc_macro_derive(Schema, attributes(scope, field))]
pub fn schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let syn::Data::Struct(item) = input.data else {
        panic!("schema can only be derived on structs");
    };

    let scope = get_class_scope(&input.attrs);

    let getters = generate_getters(item.fields, &scope);

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #(#getters)*
        }
    }
    .into()
}

fn generate_getters(fields: Fields, scope: &LitStr) -> Vec<TokenStream2> {
    fields
        .iter()
        .map(|field| {
            let (schema_class_name, schema_field_name) = get_field_info(field);
            let field_name = field.ident.clone().expect("expected field to have a name");

            let field_ty = &field.ty;

            let offset = generate_offset(scope, schema_class_name, schema_field_name);

            match field_ty {
                Type::Array(array) => {
                    if let Type::Path(elem) = array.elem.as_ref() {
                        if path_is_c_char(elem) {
                            return generate_str_array_getter(offset, field_name);
                        }
                    }

                    generate_generic_getter(offset, field_name, field_ty)
                }
                Type::Ptr(ptr) => {
                    let ptr_to = &ptr.elem;

                    if let Type::Path(ptr_to) = ptr_to.as_ref() {
                        if path_is_c_char(ptr_to) {
                            return generate_str_ptr_getter(offset, field_name);
                        }
                    }

                    generate_ptr_getter(offset, field_name, field_ty, ptr_to)
                }
                Type::Path(type_path) => {
                    if let Some(handle_type) = extract_handle_type(type_path) {
                        generate_handle_getter(offset, field_name, &handle_type)
                    } else {
                        generate_generic_getter(offset, field_name, field_ty)
                    }
                }
                _ => generate_generic_getter(offset, field_name, field_ty),
            }
        })
        .collect()
}

fn extract_handle_type(type_path: &TypePath) -> Option<Type> {
    if type_path.qself.is_none() && path_is_handle(type_path) {
        let type_params = &type_path.path.segments.first().unwrap().arguments;

        let PathArguments::AngleBracketed(generic_arg) = type_params else {
            panic!("could not get path arguments");
        };

        let generic_arg = generic_arg.args.first().unwrap();

        let GenericArgument::Type(generic) = generic_arg else {
            panic!("could not get generic argument");
        };

        Some(generic.clone())
    } else {
        None
    }
}

fn path_is_handle(type_path: &TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "Handle")
}

fn path_is_c_char(type_path: &TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "c_char")
}

fn generate_handle_getter(
    offset: TokenStream2,
    field_name: Ident,
    handle_type: &Type,
) -> TokenStream2 {
    quote! {
        pub fn #field_name(&self) -> Option<&#handle_type> {
            #offset

            ENTITY_SYSTEM.get_entity_by_handle(unsafe {
                std::mem::transmute::<_, *const Handle<#handle_type>>(
                    std::mem::transmute::<_, *const u8>(self).offset(*OFFSET as isize),
                )
                .read()
            })
        }
    }
}

fn generate_str_array_getter(offset: TokenStream2, field_name: Ident) -> TokenStream2 {
    quote! {
        pub fn #field_name(&self) -> std::borrow::Cow<'_, str> {
            #offset

            unsafe {
                std::ffi::CStr::from_ptr(
                    std::mem::transmute::<_, *const std::ffi::c_char>(self).offset(*OFFSET as isize)
                ).to_string_lossy()
            }
        }
    }
}

fn generate_generic_getter(
    offset: TokenStream2,
    field_name: Ident,
    field_ty: &Type,
) -> TokenStream2 {
    quote! {
        pub fn #field_name(&self) -> #field_ty {
            #offset

            unsafe {
                std::mem::transmute::<_, *const #field_ty>(
                    std::mem::transmute::<_, *const u8>(self).offset(*OFFSET as isize)
                ).read()
            }
        }
    }
}

fn generate_str_ptr_getter(offset: TokenStream2, field_name: Ident) -> TokenStream2 {
    quote! {
        pub fn #field_name(&self) -> std::borrow::Cow<'_, str> {
            #offset

            unsafe {
                std::ffi::CStr::from_ptr(
                    *std::mem::transmute::<_, *const *const std::ffi::c_char>(self).offset(*OFFSET as isize)
                ).to_string_lossy()
            }
        }
    }
}

fn generate_ptr_getter(
    offset: TokenStream2,
    field_name: Ident,
    field_ty: &Type,
    ptr_to: &Type,
) -> TokenStream2 {
    quote! {
        pub fn #field_name(&self) -> Option<&#ptr_to> {
            #offset

            unsafe {
                std::mem::transmute::<_, *const #field_ty>(
                    std::mem::transmute::<_, *const u8>(self).offset(*OFFSET as isize)
                ).read().as_ref()
            }
        }
    }
}

fn generate_offset(scope: &LitStr, class_name: LitStr, field_name: LitStr) -> TokenStream2 {
    quote! {
        lazy_static::lazy_static! {
            static ref OFFSET: i32 = SCHEMA_SYSTEM.find_offset(
                &encryption::x!(#scope),
                &encryption::x!(#class_name),
                &encryption::x!(#field_name)
            )
            .expect(
                &format!("{}: {}",
                    encryption::x!("could not get field offset"),
                    encryption::x!(#field_name)
                )
            );
        }
    }
}

fn get_field_info(field: &Field) -> (LitStr, LitStr) {
    let field_attr = field
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("field"))
        .expect("could not find #[field] attribute");

    let Meta::List(meta) = &field_attr.meta else {
        panic!("#[field] attribute requires 2 arguments");
    };

    let args: Vec<_> = meta
        .parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
        .expect("could not parse #[field] argument list")
        .iter()
        .take(2)
        .cloned()
        .collect();

    let class = args.get(0).expect("#[field] attribute missing class name");
    let field = args.get(1).expect("#[field] attribute missing field name");

    (class.clone(), field.clone())
}

fn get_class_scope(attributes: &[Attribute]) -> LitStr {
    let scope_attr = attributes
        .iter()
        .find(|attr| attr.path().is_ident("scope"))
        .expect("missing #[scope] attribute");

    scope_attr
        .parse_args()
        .expect("could not parse #[scope] attribute arguments")
}
