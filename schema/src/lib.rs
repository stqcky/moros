use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, DeriveInput, Field, Fields, LitStr, Meta,
    Token, Type,
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
            let (schema_class_name, schema_field_name) = get_field_info(&field);
            let field_name = field.ident.clone().expect("expected field to have a name");

            let field_ty = &field.ty;

            let offset = quote! {
                lazy_static::lazy_static! {
                    static ref OFFSET: i32 = SCHEMA_SYSTEM.find_offset(
                        &encryption::x!(#scope),
                        &encryption::x!(#schema_class_name),
                        &encryption::x!(#schema_field_name)
                    )
                    .expect(
                        &format!("{}: {}",
                            encryption::x!("could not get field offset"),
                            encryption::x!(#schema_field_name)
                        )
                    );
                }
            };

            match field_ty {
                Type::Array(array) => {
                    if let Type::Path(elem) = array.elem.as_ref() {
                        if elem.path.segments.last().is_some_and(|segment| segment.ident == "c_char") {
                            return quote! {
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
                    } 

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
                },
                Type::Ptr(ptr) => {
                    let ptr_to = &ptr.elem;

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
                },
                _ => {
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
                },
            }
        })
        .collect()
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

    let args = meta
        .parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
        .expect("could not parse #[field] argument list");

    let class = args
        .iter()
        .nth(0)
        .expect("#[field] attribute missing class name");
    let field = args
        .iter()
        .nth(1)
        .expect("#[field] attribute missing field name");

    (class.clone(), field.clone())
}

fn get_class_scope(attributes: &Vec<Attribute>) -> LitStr {
    let scope_attr = attributes
        .iter()
        .find(|attr| attr.path().is_ident("scope"))
        .expect("missing #[scope] attribute");

    scope_attr
        .parse_args()
        .expect("could not parse #[scope] attribute arguments")
}
