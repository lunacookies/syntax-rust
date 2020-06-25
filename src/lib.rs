//! Highlights Rust code.

#![warn(missing_debug_implementations, rust_2018_idioms)]

use nom::multi::many0;

mod expr;
mod ident;
mod number;
mod parser;
mod type_;

pub(crate) use expr::expr;
pub(crate) use ident::{camel_case, screaming_snake_case, snake_case};
pub(crate) use number::{binary, decimal, hexadecimal, octal};
pub(crate) use type_::type_;

#[derive(Debug)]
pub struct RustHighlighter;

type ParserOutput<'a> = nom::IResult<&'a str, Vec<dialect::HighlightedSpan<'a>>>;

impl dialect::Highlight for RustHighlighter {
    fn highlight<'input>(&self, input: &'input str) -> Vec<dialect::HighlightedSpan<'input>> {
        match many0(parser::parse)(input) {
            Ok(("", spans)) => spans.into_iter().flatten().collect(),
            _ => vec![dialect::HighlightedSpan {
                text: input,
                group: None,
            }],
        }
    }
}
