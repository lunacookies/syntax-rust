use crate::ParserOutput;
use dialect::{HighlightGroup, HighlightedSpan};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

pub(crate) fn type_(s: &str) -> ParserOutput<'_> {
    number_type(s)
}

fn number_type(s: &str) -> ParserOutput<'_> {
    map(
        alt((
            tag("u8"),
            tag("u16"),
            tag("u32"),
            tag("u64"),
            tag("u128"),
            tag("usize"),
            tag("i8"),
            tag("i16"),
            tag("i32"),
            tag("i64"),
            tag("i128"),
            tag("isize"),
            tag("f32"),
            tag("f64"),
        )),
        |s| {
            vec![HighlightedSpan {
                text: s,
                group: Some(HighlightGroup::PrimitiveTy),
            }]
        },
    )(s)
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
