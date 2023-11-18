use litrs::StringLit;
use proc_macro::TokenStream;
use proc_macro2::{Delimiter, TokenStream as TokenStream2};
use proc_macro2::{Group, Literal, TokenTree};
use quote::{quote, ToTokens};
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

    let key = &mut [0u8; 64];

    OsRng.fill_bytes(key);

    let encrypted = xor(literal.as_bytes(), key);

    let encrypted_literal = Literal::byte_string(&encrypted);
    let key_literal = Literal::byte_string(key);

    quote! {
        (#encrypted_literal, #key_literal)
    }
    .into()
}

#[proc_macro_attribute]
pub fn encrypt(_: TokenStream, tokens: TokenStream) -> TokenStream {
    parse_scope(tokens.into(), false).into()
}

static FORMAT_ARGS_IDENTIFIERS: &[&str] = &[
    "print", "println", "format", "error", "info", "trace", "debug", "bail",
];

fn parse_scope(token_stream: TokenStream2, mut format_arg_literal: bool) -> TokenStream2 {
    let mut new_stream = TokenStream2::new();
    let mut token_stream = token_stream.into_iter().peekable();

    while let Some(token_tree) = token_stream.next() {
        match token_tree {
            TokenTree::Group(ref group) => {
                if format_arg_literal {
                    new_stream.extend(encrypt_formatted_group(group.clone()));
                    format_arg_literal = false;
                } else {
                    new_stream.extend([TokenTree::from(Group::new(
                        group.delimiter(),
                        parse_scope(group.stream(), format_arg_literal),
                    ))])
                }
            }
            TokenTree::Literal(literal) => match StringLit::try_from(&literal) {
                Ok(literal)
                    if !literal.value().is_empty()
                        && literal.value() != "system"
                        && literal.value() != "fastcall" =>
                {
                    new_stream.extend(encrypt_literal(literal.value()));
                }
                _ => new_stream.extend([TokenTree::Literal(literal)]),
            },
            TokenTree::Ident(ident)
                if FORMAT_ARGS_IDENTIFIERS.contains(&ident.to_string().as_str()) =>
            {
                if let Some(TokenTree::Punct(punct)) = token_stream.peek() {
                    if punct.as_char() == '!' {
                        format_arg_literal = true;
                    }
                }
                new_stream.extend([TokenTree::Ident(ident)])
            }
            TokenTree::Punct(punct) if punct.as_char() == '#' => {
                new_stream.extend([TokenTree::Punct(punct)]);
                match token_stream.next() {
                    Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket => {
                        new_stream.extend([TokenTree::Group(group)])
                    }
                    Some(token_tree) => new_stream.extend([token_tree]),
                    None => break,
                }
            }
            t => new_stream.extend([t]),
        }
    }
    new_stream
}

fn encrypt_formatted_group(group: Group) -> TokenStream2 {
    let mut stream = group.stream().into_iter();

    let format_token = stream.next().expect("could not get format token");

    let Ok(format) = StringLit::try_from(format_token) else {
        panic!("could not parse format");
    };

    let format = format.value();

    let args_tokens = stream.skip(1);
    let mut args: Vec<TokenStream2> = vec![];

    for token in args_tokens {
        if let Some(last) = args.last_mut() {
            last.extend(token.into_token_stream());
        } else {
            args.push(token.into());
        }
    }

    let mut args = args.iter();

    let mut new_format = String::from("");
    let mut new_args: Vec<TokenStream2> = vec![];

    let mut index = 0;

    loop {
        if let Some(format_start) = format[index..].find('{') {
            if let Some(format_end) = format[index + format_start..].find('}') {
                let real_start = index + format_start;

                let literal = &format[index..format_start];

                if !literal.is_empty() {
                    new_format.push_str("{}");
                    new_args.push(encrypt_literal(literal));
                }

                let formatted = &format[real_start..real_start + format_end + 1];
                new_format.push_str(formatted);

                if let Some(arg) = args.next() {
                    new_args.push(arg.clone());
                } else if formatted
                    .chars()
                    .nth(1)
                    .is_some_and(|ch| !ch.is_alphabetic())
                {
                    panic!("expected format argument");
                }

                index += format_start + format_end + 1;
            } else {
                break;
            }
        } else {
            let remaining = &format[index..];

            if !remaining.is_empty() {
                new_format.push_str("{}");
                new_args.push(encrypt_literal(remaining));
            }

            break;
        }
    }

    quote! {
        (#new_format, #(#new_args),*)
    }
}

fn encrypt_literal(literal: &str) -> TokenStream2 {
    let key = &mut [0u8; 64];

    OsRng.fill_bytes(key);

    let encrypted = xor(&literal.to_string().into_bytes(), key);

    let encrypted_literal = Literal::byte_string(&encrypted);
    let key_literal = Literal::byte_string(key);

    quote! {
        encryption::xor::xor((#encrypted_literal, #key_literal))
    }
}
