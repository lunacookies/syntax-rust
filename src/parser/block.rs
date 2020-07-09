use super::parse_stmt;
use crate::Parser;
use dialect::HighlightGroup;

pub(super) fn parse_block(p: &mut Parser) {
    assert!(p.at(&[crate::TokenKind::OpenBrace]));
    p.push(crate::TokenKind::OpenBrace, HighlightGroup::Delimiter);

    // Keep parsing statements until we encounter a close brace.
    loop {
        if p.at(&[crate::TokenKind::CloseBrace]) {
            break;
        } else {
            parse_stmt(p);
        }
    }
    p.push(crate::TokenKind::CloseBrace, HighlightGroup::Delimiter);
}

#[cfg(test)]
mod tests {
    use super::*;
    use dialect::HighlightedSpan;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_block_with_one_expression() {
        let mut parser = Parser::new("{ a }");
        parse_block(&mut parser);

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
        parse_block(&mut parser);

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
        parse_block(&mut parser);

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
    fn parses_block_with_expression_that_throws_away_result_through_semicolon() {
        let mut parser = Parser::new("{ a(); }");
        parse_block(&mut parser);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..1,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 2..3,
                    group: HighlightGroup::FunctionCall,
                },
                HighlightedSpan {
                    range: 3..4,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 4..5,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 5..6,
                    group: HighlightGroup::Terminator,
                },
                HighlightedSpan {
                    range: 7..8,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }
}
