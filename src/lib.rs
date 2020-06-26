//! Highlights Rust code.

#![warn(missing_debug_implementations, rust_2018_idioms)]

mod lexer;

use lexer::{lex, Token, TokenKind};

#[derive(Debug)]
pub struct RustHighlighter;
