#[derive(Debug, Clone)]
pub enum Value {
    Literal(proc_macro2::Literal),
    Symbol(String),
    Unquoted(proc_macro2::TokenTree),
    Nested(proc_macro2::TokenTree),
    Expand(proc_macro2::TokenTree),
    List(Vec<Value>),
    Bracket(Vec<String>),
}
