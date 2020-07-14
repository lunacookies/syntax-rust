use dialect::{HighlightGroup, HighlightedSpan};

#[derive(Debug)]
pub(crate) struct Parser {
    pub(crate) tokens: Vec<crate::Token>,
    pub(crate) output: Vec<HighlightedSpan>,
}

impl Parser {
    pub(crate) fn new(s: &str) -> Self {
        let tokens = crate::lex(s);
        let output = Vec::with_capacity(tokens.len());

        Self { tokens, output }
    }

    pub(crate) fn next(&mut self) -> Option<crate::Token> {
        self.tokens.pop()
    }

    pub(crate) fn peek(&self) -> Option<&crate::Token> {
        self.tokens.last()
    }

    pub(crate) fn at(&self, kinds: &[crate::TokenKind]) -> bool {
        self.peek()
            .map_or(false, |token| kinds.contains(&token.kind))
    }

    pub(crate) fn at_end(&self) -> bool {
        self.peek().is_none()
    }

    pub(crate) fn eat(&mut self, group: HighlightGroup) {
        if let Some(token) = self.next() {
            self.output.push(HighlightedSpan {
                range: token.range,
                group,
            });
        }
    }

    pub(crate) fn push(&mut self, kind: crate::TokenKind, group: HighlightGroup) {
        if self.at(&[kind]) {
            let token = self.next().unwrap();

            self.output.push(HighlightedSpan {
                range: token.range,
                group,
            });
        } else {
            self.eat(HighlightGroup::Error);
        }
    }

    pub(crate) fn parse(mut self) -> Vec<HighlightedSpan> {
        while let Some(token) = self.peek() {
            match token.kind {
                crate::TokenKind::Fn => crate::grammar::parse_item(&mut self),
                _ => self.eat(HighlightGroup::Error),
            }
        }

        self.output
    }
}
