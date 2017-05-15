#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use std::str::FromStr;
use proc_macro::TokenStream;

use quote::{Tokens, ToTokens};

mod py_impl;
use py_impl::build_py_impl;


#[proc_macro_attribute]
pub fn py_impl(_: TokenStream, input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let source = input.to_string();

    // Parse the string representation into a syntax tree
    //let ast: syn::Crate = source.parse().unwrap();
    let mut ast = syn::parse_item(&source).unwrap();

    // Build the output
    let expanded = build_py_impl(&mut ast);

    // Return the generated impl as a TokenStream
    let mut tokens = Tokens::new();
    ast.to_tokens(&mut tokens);
    let s = String::from(tokens.as_str()) + expanded.as_str();

    TokenStream::from_str(s.as_str()).unwrap()
}
