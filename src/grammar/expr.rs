use crate::Parser;
use dialect::{HighlightGroup, HighlightedSpan};

pub(crate) fn parse_expr(p: &mut Parser, is_pattern: bool) {
    if let Some(token) = p.peek() {
        match token.kind {
            crate::TokenKind::Ident => {
                let var = p.next().unwrap();

                if p.at(&[crate::TokenKind::OpenParen]) {
                    p.output.push(HighlightedSpan {
                        range: var.range,
                        group: HighlightGroup::FunctionCall,
                    });

                    let open_paren = p.next().unwrap();

                    p.output.push(HighlightedSpan {
                        range: open_paren.range,
                        group: HighlightGroup::Delimiter,
                    });

                    p.push(crate::TokenKind::CloseParen, HighlightGroup::Delimiter);
                } else {
                    p.output.push(HighlightedSpan {
                        range: var.range,
                        group: if is_pattern {
                            HighlightGroup::VariableDef
                        } else {
                            HighlightGroup::VariableUse
                        },
                    });
                }
            }

            crate::TokenKind::OpenParen => parse_tuple(p, is_pattern),

            _ => p.eat(HighlightGroup::Error),
        }
    }
}

fn parse_tuple(p: &mut Parser, is_pattern: bool) {
    assert!(p.at(&[crate::TokenKind::OpenParen]));
    p.push(crate::TokenKind::OpenParen, HighlightGroup::Delimiter);

    loop {
        if p.at(&[crate::TokenKind::Comma]) {
            let comma = p.next().unwrap();

            p.output.push(HighlightedSpan {
                range: comma.range,
                group: HighlightGroup::Separator,
            });
        }

        if p.at(&[crate::TokenKind::CloseParen]) {
            break;
        }

        parse_expr(p, is_pattern);
    }

    p.push(crate::TokenKind::CloseParen, HighlightGroup::Delimiter);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_var_usage() {
        let mut parser = Parser::new("foo_bar");
        parse_expr(&mut parser, false);

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
        parse_expr(&mut parser, false);

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

    #[test]
    fn parses_empty_tuple() {
        let mut parser = Parser::new("()");
        parse_tuple(&mut parser, false);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..1,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 1..2,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_parenthesised_expression() {
        let mut parser = Parser::new("(x)");
        parse_tuple(&mut parser, false);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..1,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 1..2,
                    group: HighlightGroup::VariableUse,
                },
                HighlightedSpan {
                    range: 2..3,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_one_element_tuple() {
        let mut parser = Parser::new("(x,)");
        parse_tuple(&mut parser, false);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..1,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 1..2,
                    group: HighlightGroup::VariableUse,
                },
                HighlightedSpan {
                    range: 2..3,
                    group: HighlightGroup::Separator,
                },
                HighlightedSpan {
                    range: 3..4,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_two_element_tuple_without_trailing_comma() {
        let mut parser = Parser::new("(x, y)");
        parse_tuple(&mut parser, false);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..1,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 1..2,
                    group: HighlightGroup::VariableUse,
                },
                HighlightedSpan {
                    range: 2..3,
                    group: HighlightGroup::Separator,
                },
                HighlightedSpan {
                    range: 4..5,
                    group: HighlightGroup::VariableUse,
                },
                HighlightedSpan {
                    range: 5..6,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }
}
