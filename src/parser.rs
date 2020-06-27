use dialect::{HighlightGroup, HighlightedSpan};

#[derive(Debug)]
pub(crate) struct Parser {
    tokens: Vec<crate::Token>,
    output: Vec<HighlightedSpan>,
}

impl Iterator for Parser {
    type Item = crate::Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop()
    }
}

impl Parser {
    pub(crate) fn new(s: &str) -> Self {
        let tokens = crate::lex(s);
        let output = Vec::with_capacity(tokens.len());

        Self { tokens, output }
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
        while let Some(token) = self.next() {
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
        self.push(crate::TokenKind::Ident, HighlightGroup::FunctionDef);
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

    #[test]
    fn parses_multiple_fn_defs() {
        assert_eq!(
            Parser::new("fn foo fn bar").parse(),
            vec![
                HighlightedSpan {
                    range: 0..2,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 3..6,
                    group: HighlightGroup::FunctionDef,
                },
                HighlightedSpan {
                    range: 7..9,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 10..13,
                    group: HighlightGroup::FunctionDef,
                },
            ],
        );
    }
}
