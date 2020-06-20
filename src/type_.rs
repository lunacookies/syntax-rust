use crate::ParserOutput;
use dialect::{HighlightGroup, HighlightedSpan};
use nom::bytes::complete::tag;
use nom::combinator::map;

pub(crate) fn type_(s: &str) -> ParserOutput<'_> {
    map(tag("u32"), |s| {
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
}