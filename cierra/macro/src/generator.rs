use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use crate::value::Value;

struct GenerateCtx {
    value: Value,
    s: Ident,
    atom_break: usize,
    list_break: usize,
}

impl GenerateCtx {
    pub fn with(&self, value: Value) -> Self {
        Self { value, s: self.s.clone(), atom_break: self.atom_break, list_break: self.list_break }
    }
}

impl ToTokens for GenerateCtx {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { value, s, atom_break, list_break } = self;
        let expanded = match value {
            Value::Literal(lit) => quote! { #s.put_atom(&format!("{:?}", #lit), #atom_break); },
            Value::Symbol(name) => quote! { #s.put_atom(&format!("{}", #name), #atom_break); },
            Value::Unquoted(tt) => quote! { #s.put_atom(&(#tt).to_string(), #atom_break); },
            Value::List(elements) => {
                let pushes =
                    elements.iter().map(|elem| self.with(elem.clone()).into_token_stream());
                quote! {
                    #s.begin_list(#list_break);
                    #(#pushes)*
                    #s.end_list();
                }
            }
            Value::Bracket(elements) => {
                let count = elements.len();
                let elements = elements.iter().enumerate().map(|(idx, elem)| {
                    match (idx == 0, idx == count - 1) {
                        (true, true) => format!("[{}]", elem),
                        (true, false) => format!("[{}", elem),
                        (false, true) => format!("{}]", elem),
                        (false, false) => elem.to_string(),
                    }
                });
                quote! {
                    #( #s.put_atom(#elements, #atom_break); )*
                }
            }
            Value::Nested(tt) => quote! {
                <sexp::SexpDisplay>::fmt(&#tt, #s);
            },
            Value::Expand(tt) => quote! {
                #[allow(unused_parens)]
                for elem in #tt {
                    <sexp::SexpDisplay>::fmt(elem, #s);
                }
            },
        };
        tokens.extend(expanded);
        // tokens.extend(quote! {
        //     |#s: &mut ::sise::Serializer| {
        //         #expanded
        //     }
        // });
    }
}

pub fn generate(s: Ident, value: Value, atom_break: usize, list_break: usize) -> TokenStream {
    GenerateCtx { value, s, atom_break, list_break }.into_token_stream()
}
