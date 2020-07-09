mod block;
mod expr;
mod stmt;

use block::parse_block;
use dialect::{HighlightGroup, HighlightedSpan};
use expr::parse_expr;
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
                crate::TokenKind::Fn => self.parse_fn_def(),
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

    fn parse_fn_def(&mut self) {
        assert!(self.at(&[crate::TokenKind::Fn]));

        self.push(crate::TokenKind::Fn, HighlightGroup::OtherKeyword);
        self.push(crate::TokenKind::Ident, HighlightGroup::FunctionDef);
        self.push(crate::TokenKind::OpenParen, HighlightGroup::Delimiter);
        self.push(crate::TokenKind::CloseParen, HighlightGroup::Delimiter);

        if self.at(&[crate::TokenKind::ThinArrow]) {
            let thin_arrow = self.next().unwrap();

            if self.at(&[crate::TokenKind::TypeIdent]) {
                let type_ = self.next().unwrap();
                self.output.push(HighlightedSpan {
                    range: thin_arrow.range,
                    group: HighlightGroup::Separator,
                });
                self.output.push(HighlightedSpan {
                    range: type_.range,
                    group: HighlightGroup::TyUse,
                });
            } else {
                // If we don’t have a type after the arrow, then the arrow is parsed as an error.
                self.output.push(HighlightedSpan {
                    range: thin_arrow.range,
                    group: HighlightGroup::Error,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_fn_def() {
        assert_eq!(
            Parser::new("fn frobnicate()").parse(),
            vec![
                HighlightedSpan {
                    range: 0..2,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 3..13,
                    group: HighlightGroup::FunctionDef,
                },
                HighlightedSpan {
                    range: 13..14,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 14..15,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_multiple_fn_defs() {
        assert_eq!(
            Parser::new("fn foo() fn bar()").parse(),
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
                    range: 6..7,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 7..8,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 9..11,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 12..15,
                    group: HighlightGroup::FunctionDef,
                },
                HighlightedSpan {
                    range: 15..16,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 16..17,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_fn_def_with_return_type() {
        assert_eq!(
            Parser::new("fn f() -> T").parse(),
            vec![
                HighlightedSpan {
                    range: 0..2,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 3..4,
                    group: HighlightGroup::FunctionDef,
                },
                HighlightedSpan {
                    range: 4..5,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 5..6,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 7..9,
                    group: HighlightGroup::Separator,
                },
                HighlightedSpan {
                    range: 10..11,
                    group: HighlightGroup::TyUse
                },
            ],
        );
    }

    #[test]
    fn return_type_arrow_without_type_is_error() {
        assert_eq!(
            Parser::new("fn f() ->").parse(),
            vec![
                HighlightedSpan {
                    range: 0..2,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 3..4,
                    group: HighlightGroup::FunctionDef,
                },
                HighlightedSpan {
                    range: 4..5,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 5..6,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 7..9,
                    group: HighlightGroup::Error,
                },
            ],
        );
    }
}
