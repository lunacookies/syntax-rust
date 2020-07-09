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
