use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use rand::{rngs::OsRng, RngCore};
use syn::{parse_macro_input, Lit};

fn xor(source: &[u8], key: &[u8]) -> Vec<u8> {
    source
        .iter()
        .zip(key.iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}

#[proc_macro]
pub fn x(tokens: TokenStream) -> TokenStream {
    let literal = match parse_macro_input!(tokens as Lit) {
        Lit::Str(lit) => lit,
        _ => panic!("expected string literal"),
    }
    .value();

    let key = &mut [0u8; 32];

    OsRng.fill_bytes(key);

    let encrypted = xor(literal.as_bytes(), key);

    let encrypted_literal = Literal::byte_string(&encrypted);
    let key_literal = Literal::byte_string(key);

    quote! {
        (#encrypted_literal, #key_literal)
    }
    .into()
}
