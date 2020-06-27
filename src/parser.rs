use dialect::{HighlightGroup, HighlightedSpan};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Parser {
    tokens: Vec<crate::Token>,
    output: Vec<HighlightedSpan>,
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

    pub(crate) fn parse(mut self) -> Vec<HighlightedSpan> {
        if let Some(token) = self.next() {
            match token.kind {
                crate::TokenKind::Fn => {
                    self.output.push(HighlightedSpan {
                        group: HighlightGroup::OtherKeyword,
                        range: token.range,
                    });
                    self.parse_fn_def();
                }
                _ => self.output.push(HighlightedSpan {
                    group: HighlightGroup::Error,
                    range: token.range,
                }),
            }
        }

        self.output
    }

    fn parse_fn_def(&mut self) {
        if let Some(token) = self.next() {
            let group = match token.kind {
                crate::TokenKind::Ident => HighlightGroup::FunctionDef,
                _ => HighlightGroup::Error,
            };

            self.output.push(HighlightedSpan {
                range: token.range,
                group,
            });
        }
    }
}

impl dialect::Highlight for Parser {
    fn highlight(&self, input: &str) -> Vec<dialect::HighlightedSpan> {
        Parser::new(input).parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_fn_def() {
        assert_eq!(
            Parser::new("fn frobnicate").parse(),
            vec![
                HighlightedSpan {
                    range: 0..2,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 3..13,
                    group: HighlightGroup::FunctionDef,
                },
            ],
        );
    }
}
