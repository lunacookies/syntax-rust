use dialect::{HighlightGroup, HighlightedSpan};

#[derive(Debug)]
pub(crate) struct Parser {
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

    fn at(&self, kind: crate::TokenKind) -> bool {
        self.peek().map_or(false, |token| token.kind == kind)
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
        self.push(crate::TokenKind::OpenParen, HighlightGroup::Delimiter);
        self.push(crate::TokenKind::CloseParen, HighlightGroup::Delimiter);

        if self.at(crate::TokenKind::ThinArrow) {
            let thin_arrow = self.next().unwrap();

            if self.at(crate::TokenKind::TypeIdent) {
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

    fn parse_block(&mut self) {
        self.push(crate::TokenKind::OpenBrace, HighlightGroup::Delimiter);

        // Keep parsing statements until we encounter a close brace.
        loop {
            if self.at(crate::TokenKind::CloseBrace) {
                break;
            } else {
                self.parse_stmt();
            }
        }
        self.push(crate::TokenKind::CloseBrace, HighlightGroup::Delimiter);
    }

    fn parse_stmt(&mut self) {
        match self.peek() {
            Some(crate::Token {
                kind: crate::TokenKind::Let,
                ..
            }) => {
                let let_ = self.next().unwrap();

                self.output.push(HighlightedSpan {
                    range: let_.range,
                    group: HighlightGroup::OtherKeyword,
                });

                self.parse_expr(true);
                self.push(crate::TokenKind::Equals, HighlightGroup::AssignOper);
                self.parse_expr(false);

                self.push(crate::TokenKind::Semi, HighlightGroup::Terminator);
            }
            _ => {
                self.parse_expr(false);

                // Only parse semicolon if the next token is not a close brace -- if it is, then
                // that means we are at the end of a block and as such don’t require a semicolon.
                match self.peek() {
                    Some(crate::Token {
                        kind: crate::TokenKind::CloseBrace,
                        ..
                    }) => {}
                    _ => self.push(crate::TokenKind::Semi, HighlightGroup::Terminator),
                }
            }
        }
    }

    fn parse_expr(&mut self, is_pattern: bool) {
        if self.at(crate::TokenKind::Ident) {
            let var = self.next().unwrap();

            if self.at(crate::TokenKind::OpenParen) {
                self.output.push(HighlightedSpan {
                    range: var.range,
                    group: HighlightGroup::FunctionCall,
                });

                let open_paren = self.next().unwrap();

                self.output.push(HighlightedSpan {
                    range: open_paren.range,
                    group: HighlightGroup::Delimiter,
                });

                self.push(crate::TokenKind::CloseParen, HighlightGroup::Delimiter);
            } else {
                self.output.push(HighlightedSpan {
                    range: var.range,
                    group: if is_pattern {
                        HighlightGroup::VariableDef
                    } else {
                        HighlightGroup::VariableUse
                    },
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

    #[test]
    fn parses_block_with_one_expression() {
        let mut parser = Parser::new("{ a }");
        parser.parse_block();

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..1,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 2..3,
                    group: HighlightGroup::VariableUse,
                },
                HighlightedSpan {
                    range: 4..5,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_block_with_one_statement() {
        let mut parser = Parser::new("{ let x = y; }");
        parser.parse_block();

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..1,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 2..5,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 6..7,
                    group: HighlightGroup::VariableDef,
                },
                HighlightedSpan {
                    range: 8..9,
                    group: HighlightGroup::AssignOper,
                },
                HighlightedSpan {
                    range: 10..11,
                    group: HighlightGroup::VariableUse,
                },
                HighlightedSpan {
                    range: 11..12,
                    group: HighlightGroup::Terminator,
                },
                HighlightedSpan {
                    range: 13..14,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parse_block_with_statement_and_expression() {
        let mut parser = Parser::new("{ let a = foo(); a }");
        parser.parse_block();

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..1,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 2..5,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 6..7,
                    group: HighlightGroup::VariableDef,
                },
                HighlightedSpan {
                    range: 8..9,
                    group: HighlightGroup::AssignOper,
                },
                HighlightedSpan {
                    range: 10..13,
                    group: HighlightGroup::FunctionCall,
                },
                HighlightedSpan {
                    range: 13..14,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 14..15,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 15..16,
                    group: HighlightGroup::Terminator,
                },
                HighlightedSpan {
                    range: 17..18,
                    group: HighlightGroup::VariableUse,
                },
                HighlightedSpan {
                    range: 19..20,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_let_statement() {
        let mut parser = Parser::new("let x = y;");
        parser.parse_stmt();

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..3,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 4..5,
                    group: HighlightGroup::VariableDef,
                },
                HighlightedSpan {
                    range: 6..7,
                    group: HighlightGroup::AssignOper,
                },
                HighlightedSpan {
                    range: 8..9,
                    group: HighlightGroup::VariableUse,
                },
                HighlightedSpan {
                    range: 9..10,
                    group: HighlightGroup::Terminator,
                },
            ],
        );
    }

    #[test]
    fn parses_var_usage() {
        let mut parser = Parser::new("foo_bar");
        parser.parse_expr(false);

        assert_eq!(
            parser.output,
            vec![HighlightedSpan {
                range: 0..7,
                group: HighlightGroup::VariableUse,
            }],
        );
    }

    #[test]
    fn parses_function_call() {
        let mut parser = Parser::new("f()");
        parser.parse_expr(false);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..1,
                    group: HighlightGroup::FunctionCall,
                },
                HighlightedSpan {
                    range: 1..2,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 2..3,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }
}
