mod fn_;
mod struct_;
mod trait_;

use crate::Parser;
use fn_::parse_fn;
use struct_::parse_struct;
use trait_::parse_trait;

pub(crate) fn parse_item(p: &mut Parser) {
    if let Some(token) = p.peek() {
        match token.kind {
            crate::TokenKind::Fn => parse_fn(p),
            crate::TokenKind::Struct => parse_struct(p),
            crate::TokenKind::Trait => parse_trait(p),

            // parse_item should only be called if the next token has been verified to be able to
            // start an item.
            _ => panic!(),
        }
    }
}

// These tests just check if each item’s parser has been hooked in correctly by looking at the first
// HighlightedSpan found, rather than checking the entire parser output.
#[cfg(test)]
mod tests {
    use super::*;
    use dialect::HighlightGroup;
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
}
