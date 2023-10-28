use proc_macro::TokenStream;

mod cstruct;
mod vfunc;
mod vmt;

#[proc_macro_derive(C, attributes(str, ptr))]
pub fn cstruct(input: TokenStream) -> TokenStream {
    cstruct::cstruct_impl(input)
}

#[proc_macro_attribute]
pub fn vmt(_attr: TokenStream, item: TokenStream) -> TokenStream {
    vmt::vmt_impl(item)
}

#[proc_macro_attribute]
pub fn vfunc(attr: TokenStream, item: TokenStream) -> TokenStream {
    vfunc::vfunc_impl(attr, item)
}
