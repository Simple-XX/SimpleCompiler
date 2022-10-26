extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    Token,
};

mod generator;
mod parser;
mod value;

struct Input {
    s: Ident,
    _comma: Token![,],
    e: proc_macro2::TokenStream,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { s: input.parse()?, _comma: input.parse()?, e: input.parse()? })
    }
}

/// Example of [function-like procedural macro][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros
#[proc_macro]
pub fn sexp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);

    let tokens = match parser::parse(input.e) {
        Ok(value) => generator::generate(input.s, value, usize::MAX, 0),
        Err(e) => {
            let msg = format!("could not parse s-expression: {:?}", e);
            quote! { compile_error!(#msg) }
        }
    };

    tokens.into()
}
