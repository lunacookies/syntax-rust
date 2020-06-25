use crate::{decimal, octal, snake_case, ParserOutput};
use dialect::{HighlightGroup, HighlightedSpan};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

pub(crate) fn expr(s: &str) -> ParserOutput<'_> {
    alt((number, var))(s)
}

fn number(s: &str) -> ParserOutput<'_> {
    alt((octal_lit, decimal_lit))(s)
}

fn octal_lit(s: &str) -> ParserOutput<'_> {
    let (s, prefix) = tag("0o")(s)?;
    let (s, digits) = octal(s)?;

    Ok((
        s,
        vec![
            HighlightedSpan {
                text: prefix,
                group: Some(HighlightGroup::Number),
            },
            HighlightedSpan {
                text: digits,
                group: Some(HighlightGroup::Number),
            },
        ],
    ))
}

fn decimal_lit(s: &str) -> ParserOutput<'_> {
    map(decimal, |s| {
        vec![HighlightedSpan {
            text: s,
            group: Some(HighlightGroup::Number),
        }]
    })(s)
}

fn var(s: &str) -> ParserOutput<'_> {
    map(snake_case, |s| {
        vec![HighlightedSpan {
            text: s,
            group: Some(HighlightGroup::VariableUse),
        }]
    })(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_variable() {
        assert_eq!(
            expr("foobar"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "foobar",
                    group: Some(HighlightGroup::VariableUse)
                }]
            ))
        );
    }

    #[test]
    fn parse_octal_literal() {
        assert_eq!(
            expr("0o1234567"),
            Ok((
                "",
                vec![
                    HighlightedSpan {
                        text: "0o",
                        group: Some(HighlightGroup::Number)
                    },
                    HighlightedSpan {
                        text: "1234567",
                        group: Some(HighlightGroup::Number)
                    }
                ]
            ))
        );
    }

    #[test]
    fn parse_decimal_literal() {
        assert_eq!(
            expr("123456789"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "123456789",
                    group: Some(HighlightGroup::Number)
                }]
            ))
        );
    }
}
