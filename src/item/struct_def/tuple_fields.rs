use {
    crate::{
        utils::{comma_separated, take_whitespace0},
        ParseResult,
    },
    nom::bytes::complete::tag,
};

const FIELDS_START: &str = "(";
const FIELDS_END: &str = ")";

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, open_paren) = tag(FIELDS_START)(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    // Fields of a tuple struct are simply types.
    let (s, mut fields) = comma_separated(&crate::ty, FIELDS_END)(s)?;
    let (s, fields_space) = take_whitespace0(s)?;

    let (s, close_paren) = tag(FIELDS_END)(s)?;
    let (s, close_paren_space) = take_whitespace0(s)?;

    let (s, semicolon) = tag(";")(s)?;

    let mut output = vec![
        dialect::HighlightedSpan {
            text: open_paren,
            group: Some(dialect::HighlightGroup::Delimiter),
        },
        dialect::HighlightedSpan {
            text: open_paren_space,
            group: None,
        },
    ];

    output.append(&mut fields);

    output.extend_from_slice(&[
        dialect::HighlightedSpan {
            text: fields_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: close_paren,
            group: Some(dialect::HighlightGroup::Delimiter),
        },
        dialect::HighlightedSpan {
            text: close_paren_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: semicolon,
            group: Some(dialect::HighlightGroup::Terminator),
        },
    ]);

    Ok((s, output))
}
