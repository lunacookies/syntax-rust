use crate::{camel_case, ParserOutput};
use dialect::{HighlightGroup, HighlightedSpan};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

pub(crate) fn type_(s: &str) -> ParserOutput<'_> {
    alt((number_type, bool_type, str_type, non_primitive_type))(s)
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

fn bool_type(s: &str) -> ParserOutput<'_> {
    map(tag("bool"), |s| {
        vec![HighlightedSpan {
            text: s,
            group: Some(HighlightGroup::PrimitiveTy),
        }]
    })(s)
}

fn str_type(s: &str) -> ParserOutput<'_> {
    map(tag("str"), |s| {
        vec![HighlightedSpan {
            text: s,
            group: Some(HighlightGroup::PrimitiveTy),
        }]
    })(s)
}

fn non_primitive_type(s: &str) -> ParserOutput<'_> {
    map(camel_case, |s| {
        vec![HighlightedSpan {
            text: s,
            group: Some(HighlightGroup::TyUse),
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

    #[test]
    fn parse_bool() {
        assert_eq!(
            type_("bool"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "bool",
                    group: Some(HighlightGroup::PrimitiveTy)
                }]
            ))
        );
    }

    #[test]
    fn parse_str() {
        assert_eq!(
            type_("str"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "str",
                    group: Some(HighlightGroup::PrimitiveTy)
                }]
            ))
        );
    }

    #[test]
    fn parse_non_primitive_type() {
        assert_eq!(
            type_("String"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "String",
                    group: Some(HighlightGroup::TyUse)
                }]
            ))
        );
    }
}
