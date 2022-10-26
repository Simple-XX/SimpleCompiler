use proc_macro2::{Delimiter, Punct, Spacing, TokenStream, TokenTree};

use crate::value::Value;

#[derive(Debug)]
struct Parser {
    tokens: Vec<TokenTree>,
    index: usize,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedDelimiter(Delimiter),
    UnexpectedEnd,
}

impl Parser {
    fn new(tokens: Vec<TokenTree>) -> Self {
        Self { tokens, index: 0 }
    }

    fn next_token(&mut self) -> Option<&TokenTree> {
        if self.index == self.tokens.len() {
            return None;
        }
        let token = &self.tokens[self.index];
        self.index += 1;
        Some(token)
    }

    fn token(&mut self) -> Result<&TokenTree, ParseError> {
        self.next_token().ok_or(ParseError::UnexpectedEnd)
    }

    fn peek(&mut self) -> Option<&TokenTree> {
        if self.index == self.tokens.len() {
            return None;
        }
        Some(&self.tokens[self.index])
    }

    fn parse_puncts(&mut self, first: &Punct) -> Result<String, ParseError> {
        let mut buf = String::from(first.as_char());
        let mut spacing = first.spacing();
        while spacing == Spacing::Joint {
            let punct = self.token()?;
            match punct {
                TokenTree::Punct(punct) => {
                    buf.push(punct.as_char());
                    spacing = punct.spacing();
                }
                _ => break,
            }
        }
        Ok(buf)
    }

    fn parse(&mut self) -> Result<Value, ParseError> {
        let first_punct = match self.token()? {
            TokenTree::Punct(punct) => punct,
            TokenTree::Literal(literal) => return Ok(Value::Literal(literal.clone())),
            TokenTree::Ident(ident) => return Ok(Value::Symbol(ident.to_string())),
            TokenTree::Group(group) => {
                return match group.delimiter() {
                    Delimiter::Parenthesis => parse_list(group.stream()),
                    Delimiter::Bracket => parse_bracket(group.stream()),
                    delim => Err(ParseError::UnexpectedDelimiter(delim)),
                };
            }
        }
        .clone();
        match self.parse_puncts(&first_punct)?.as_str() {
            "," => Ok(Value::Unquoted(self.token()?.clone())),
            "%" => Ok(Value::Nested(self.token()?.clone())),
            "..." => Ok(Value::Expand(self.token()?.clone())),
            c => Ok(Value::Symbol(c.to_string())),
        }
    }
}

fn parse_bracket(tokens: TokenStream) -> Result<Value, ParseError> {
    let mut elements = vec![];
    let mut parser = Parser::new(tokens.into_iter().collect());
    while parser.peek().is_some() {
        elements.push(parser.parse()?);
    }
    Ok(Value::Bracket(
        elements
            .into_iter()
            .filter_map(|elem| match elem {
                Value::Symbol(x) => Some(x),
                _ => None,
            })
            .collect(),
    ))
}

fn parse_list(tokens: TokenStream) -> Result<Value, ParseError> {
    let mut elements = vec![];
    let mut parser = Parser::new(tokens.into_iter().collect());
    while parser.peek().is_some() {
        elements.push(parser.parse()?);
    }
    Ok(Value::List(elements))
}

pub fn parse(tokens: TokenStream) -> Result<Value, ParseError> {
    let mut parser = Parser::new(tokens.into_iter().collect());
    parser.parse()
}
