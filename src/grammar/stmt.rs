use crate::grammar::parse_expr;
use crate::Parser;
use dialect::{HighlightGroup, HighlightedSpan};

pub(crate) fn parse_stmt(p: &mut Parser) {
    match p.peek() {
        Some(crate::Token {
            kind: crate::TokenKind::Let,
            ..
        }) => {
            p.eat(HighlightGroup::OtherKeyword);

            parse_expr(p, true);
            p.push(crate::TokenKind::Equals, HighlightGroup::AssignOper);
            parse_expr(p, false);

            p.push(crate::TokenKind::Semi, HighlightGroup::Terminator);
        }
        _ => {
            parse_expr(p, false);

            // Only parse semicolon if the next token is not a close brace -- if it is, then
            // that means we are at the end of a block and as such don’t require a semicolon.
            if !p.at(&[crate::TokenKind::CloseBrace]) {
                p.push(crate::TokenKind::Semi, HighlightGroup::Terminator);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_let_statement() {
        let mut parser = Parser::new("let x = y;");
        parse_stmt(&mut parser);

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
}
