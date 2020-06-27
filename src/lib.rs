//! Highlights Rust code.

#![warn(missing_debug_implementations, rust_2018_idioms)]

mod lexer;
mod parser;

use lexer::{lex, Token, TokenKind};
use parser::Parser;

#[derive(Debug)]
pub struct RustHighlighter;

impl dialect::Highlight for RustHighlighter {
    fn highlight(&self, input: &str) -> Vec<dialect::HighlightedSpan> {
        Parser::new(input).parse()
    }
}
