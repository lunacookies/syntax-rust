use crate::ParserOutput;
use dialect::HighlightedSpan;
use nom::bytes::complete::take_while;
use nom::combinator::map;

pub(crate) fn parse(s: &str) -> ParserOutput<'_> {
    if s == "" {
        return Ok(("", vec![]));
    }

    map(
        take_while(|c| c == ' ' || c == '\t' || c == '\n' || c == '\r'),
        |s| {
            vec![HighlightedSpan {
                text: s,
                group: None,
            }]
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nothing() {
        assert_eq!(parse(""), Ok(("", vec![])));
    }

    #[test]
    fn parse_spaces() {
        assert_eq!(
            parse("   "),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "   ",
                    group: None
                }]
            ))
        );
    }

    #[test]
    fn parse_tabs() {
        assert_eq!(
            parse("\t\t"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "\t\t",
                    group: None
                }]
            ))
        );
    }

    #[test]
    fn parse_line_feeds() {
        assert_eq!(
            parse("\n\n\n"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "\n\n\n",
                    group: None
                }]
            ))
        );
    }

    #[test]
    fn parse_carriage_returns() {
        assert_eq!(
            parse("\r\r"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "\r\r",
                    group: None
                }]
            ))
        );
    }
}
