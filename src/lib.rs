//! Highlights Rust code.

#![warn(missing_debug_implementations, rust_2018_idioms)]

use nom::multi::many0;

mod parser;

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
