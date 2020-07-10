mod fn_;
mod struct_;
mod trait_;

use crate::Parser;
use dialect::HighlightGroup;
use fn_::parse_fn;
use struct_::parse_struct;
use trait_::parse_trait;

pub(crate) fn parse_item(p: &mut Parser) {
    while let Some(token) = p.peek() {
        match token.kind {
            crate::TokenKind::Fn => {
                parse_fn(p);
                break;
            }
            crate::TokenKind::Struct => {
                parse_struct(p);
                break;
            }
            crate::TokenKind::Trait => {
                parse_trait(p);
                break;
            }

            crate::TokenKind::CloseBrace => {
                break;
            }
            _ => p.eat(HighlightGroup::Error),
        }
    }
}

// Most of these tests just check if each item’s parser has been hooked in correctly by looking at
// the first HighlightedSpan found, rather than checking the entire parser output.
#[cfg(test)]
mod tests {
    use super::*;
    use dialect::HighlightedSpan;
    use pretty_assertions::assert_eq;

    fn test(input: &str, expected_group: HighlightGroup) {
        let mut parser = Parser::new(input);
        parse_item(&mut parser);

        assert_eq!(parser.output[0].group, expected_group);
    }

    #[test]
    fn parses_function() {
        test("fn a() {}", HighlightGroup::OtherKeyword);
    }

    #[test]
    fn parses_struct() {
        test("struct Parser;", HighlightGroup::OtherKeyword);
    }

    #[test]
    fn parses_trait() {
        test("trait Marker {}", HighlightGroup::OtherKeyword);
    }

    #[test]
    fn eats_errors_until_finds_valid_keyword() {
        let mut parser = Parser::new("error blah fn");
        parse_item(&mut parser);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..5,
                    group: HighlightGroup::Error,
                },
                HighlightedSpan {
                    range: 6..10,
                    group: HighlightGroup::Error,
                },
                HighlightedSpan {
                    range: 11..13,
                    group: HighlightGroup::OtherKeyword,
                },
            ],
        );
    }

    #[test]
    fn eats_errors_until_close_brace() {
        let mut parser = Parser::new("invalid item }");
        parse_item(&mut parser);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..7,
                    group: HighlightGroup::Error,
                },
                HighlightedSpan {
                    range: 8..12,
                    group: HighlightGroup::Error,
                },
            ],
        );
    }
}
