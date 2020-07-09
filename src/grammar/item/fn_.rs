use crate::grammar::parse_block;
use crate::Parser;
use dialect::{HighlightGroup, HighlightedSpan};

pub(super) fn parse_fn(p: &mut Parser) {
    assert!(p.at(&[crate::TokenKind::Fn]));

    p.push(crate::TokenKind::Fn, HighlightGroup::OtherKeyword);
    p.push(crate::TokenKind::Ident, HighlightGroup::FunctionDef);
    p.push(crate::TokenKind::OpenParen, HighlightGroup::Delimiter);
    p.push(crate::TokenKind::CloseParen, HighlightGroup::Delimiter);

    if p.at(&[crate::TokenKind::ThinArrow]) {
        let thin_arrow = p.next().unwrap();

        if p.at(&[crate::TokenKind::TypeIdent]) {
            let type_ = p.next().unwrap();
            p.output.push(HighlightedSpan {
                range: thin_arrow.range,
                group: HighlightGroup::Separator,
            });
            p.output.push(HighlightedSpan {
                range: type_.range,
                group: HighlightGroup::TyUse,
            });
        } else {
            // If we don’t have a type after the arrow, then the arrow is parsed as an error.
            p.output.push(HighlightedSpan {
                range: thin_arrow.range,
                group: HighlightGroup::Error,
            });
        }
    }

    if let Some(token) = p.peek() {
        match token.kind {
            crate::TokenKind::Semi => p.push(crate::TokenKind::Semi, HighlightGroup::Terminator),
            crate::TokenKind::OpenBrace => parse_block(p),
            _ => {
                let token = p.next().unwrap();

                p.output.push(HighlightedSpan {
                    range: token.range,
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
    fn parses_fn_def_with_empty_body() {
        let mut parser = Parser::new("fn frobnicate() {}");
        parse_fn(&mut parser);

        assert_eq!(
            parser.output,
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
                HighlightedSpan {
                    range: 16..17,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 17..18,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_fn_def_with_semicolon_instead_of_body() {
        let mut parser = Parser::new("fn frobnicate();");
        parse_fn(&mut parser);

        assert_eq!(
            parser.output,
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
                HighlightedSpan {
                    range: 15..16,
                    group: HighlightGroup::Terminator,
                },
            ],
        );
    }

    #[test]
    fn parses_fn_def_with_return_type() {
        let mut parser = Parser::new("fn f() -> T {}");
        parse_fn(&mut parser);

        assert_eq!(
            parser.output,
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
                HighlightedSpan {
                    range: 12..13,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 13..14,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn return_type_arrow_without_type_is_error() {
        let mut parser = Parser::new("fn f() -> {}");
        parse_fn(&mut parser);

        assert_eq!(
            parser.output,
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
                HighlightedSpan {
                    range: 10..11,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 11..12,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }
}
