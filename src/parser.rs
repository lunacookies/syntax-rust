mod block;
mod expr;
mod item;
mod stmt;

use block::parse_block;
use dialect::{HighlightGroup, HighlightedSpan};
use expr::parse_expr;
use item::parse_item;
use stmt::parse_stmt;

#[derive(Debug)]
pub(crate) struct Parser {
    pub(self) tokens: Vec<crate::Token>,
    pub(self) output: Vec<HighlightedSpan>,
}

impl Parser {
    pub(crate) fn new(s: &str) -> Self {
        let tokens = crate::lex(s);
        let output = Vec::with_capacity(tokens.len());

        Self { tokens, output }
    }

    fn next(&mut self) -> Option<crate::Token> {
        self.tokens.pop()
    }

    fn peek(&self) -> Option<&crate::Token> {
        self.tokens.last()
    }

    fn at(&self, kinds: &[crate::TokenKind]) -> bool {
        self.peek()
            .map_or(false, |token| kinds.contains(&token.kind))
    }

    fn push(&mut self, kind: crate::TokenKind, group: HighlightGroup) {
        if let Some(token) = self.next() {
            let group = if kind == token.kind {
                group
            } else {
                HighlightGroup::Error
            };

            self.output.push(HighlightedSpan {
                range: token.range,
                group,
            });
        }
    }

    pub(crate) fn parse(mut self) -> Vec<HighlightedSpan> {
        while let Some(token) = self.peek() {
            match token.kind {
                crate::TokenKind::Fn => parse_item(&mut self),
                _ => {
                    let range = token.range.clone();
                    self.output.push(HighlightedSpan {
                        group: HighlightGroup::Error,
                        range,
                    })
                }
            }
        }

        self.output
    }
}
