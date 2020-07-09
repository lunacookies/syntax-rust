use crate::Parser;
use dialect::HighlightGroup;

pub(super) fn parse_struct(p: &mut Parser) {
    assert!(p.at(&[crate::TokenKind::Struct]));

    p.push(crate::TokenKind::Struct, HighlightGroup::OtherKeyword);
    p.push(crate::TokenKind::TypeIdent, HighlightGroup::TyDef);
    p.push(crate::TokenKind::Semi, HighlightGroup::Terminator);
}

#[cfg(test)]
mod tests {
    use super::*;
    use dialect::HighlightedSpan;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_unit_struct() {
        let mut parser = Parser::new("struct T;");
        parse_struct(&mut parser);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..6,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 7..8,
                    group: HighlightGroup::TyDef,
                },
                HighlightedSpan {
                    range: 8..9,
                    group: HighlightGroup::Terminator,
                },
            ],
        );
    }
}
