use crate::{binary, decimal, hexadecimal, octal, snake_case, ParserOutput};
use dialect::{HighlightGroup, HighlightedSpan};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

pub(crate) fn expr(s: &str) -> ParserOutput<'_> {
    alt((hexadecimal_lit, octal_lit, binary_lit, decimal_lit, var))(s)
}

fn hexadecimal_lit(s: &str) -> ParserOutput<'_> {
    number("0x", hexadecimal, s)
}

fn octal_lit(s: &str) -> ParserOutput<'_> {
    number("0o", octal, s)
}

fn binary_lit(s: &str) -> ParserOutput<'_> {
    number("0b", binary, s)
}

fn decimal_lit(s: &str) -> ParserOutput<'_> {
    number("", decimal, s)
}

fn number<'a>(
    prefix: &str,
    digits: impl Fn(&str) -> nom::IResult<&str, &str>,
    s: &'a str,
) -> ParserOutput<'a> {
    let (s, prefix) = tag(prefix)(s)?;
    let (s, digits) = digits(s)?;

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
                    group: Some(HighlightGroup::VariableUse),
                }],
            ))
        );
    }

    #[test]
    fn parse_hexadecimal_literal() {
        assert_eq!(
            expr("0xDEADBEEF"),
            Ok((
                "",
                vec![
                    HighlightedSpan {
                        text: "0x",
                        group: Some(HighlightGroup::Number),
                    },
                    HighlightedSpan {
                        text: "DEADBEEF",
                        group: Some(HighlightGroup::Number)
                    },
                ],
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
                        group: Some(HighlightGroup::Number),
                    },
                    HighlightedSpan {
                        text: "1234567",
                        group: Some(HighlightGroup::Number),
                    },
                ],
            ))
        );
    }

    #[test]
    fn parse_binary_literal() {
        assert_eq!(
            expr("0b010101"),
            Ok((
                "",
                vec![
                    HighlightedSpan {
                        text: "0b",
                        group: Some(HighlightGroup::Number),
                    },
                    HighlightedSpan {
                        text: "010101",
                        group: Some(HighlightGroup::Number),
                    },
                ],
            ))
        );
    }

    #[test]
    fn parse_decimal_literal() {
        assert_eq!(
            expr("123456789"),
            Ok((
                "",
                vec![
                    // Two HighlightedSpans because the parsing of *all* number literals is
                    // generalised with a prefix -- decimal doesn’t have a prefix, so an empty
                    // string is used instead.
                    HighlightedSpan {
                        text: "",
                        group: Some(HighlightGroup::Number),
                    },
                    HighlightedSpan {
                        text: "123456789",
                        group: Some(HighlightGroup::Number),
                    },
                ],
            ))
        );
    }
}
