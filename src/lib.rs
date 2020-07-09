//! Highlights Rust code.

#![warn(missing_debug_implementations, rust_2018_idioms)]

mod grammar;
mod lexer;
mod parser;

use lexer::{lex, Token, TokenKind};
use parser::Parser;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Highlighter;

impl dialect::Highlight for Highlighter {
    fn highlight(&self, input: &str) -> Vec<dialect::HighlightedSpan> {
        Parser::new(input).parse()
    }
}
