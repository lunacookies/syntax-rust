use crate::ParserOutput;
use dialect::{HighlightGroup, HighlightedSpan};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

pub(crate) fn type_(s: &str) -> ParserOutput<'_> {
    map(alt((tag("u32"), tag("i32"))), |s| {
        vec![HighlightedSpan {
            text: s,
            group: Some(HighlightGroup::PrimitiveTy),
        }]
    })(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_u32() {
        assert_eq!(
            type_("u32"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "u32",
                    group: Some(HighlightGroup::PrimitiveTy)
                }]
            ))
        );
    }

    #[test]
    fn parse_i32() {
        assert_eq!(
            type_("i32"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "i32",
                    group: Some(HighlightGroup::PrimitiveTy)
                }]
            ))
        );
    }
}
