use {
    crate::{
        utils::{comma_separated, snake_case, take_whitespace0},
        ParseResult,
    },
    nom::bytes::complete::tag,
};

const FIELDS_START: &str = "{";
const FIELDS_END: &str = "}";

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, open_brace) = tag(FIELDS_START)(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    let (s, mut fields) = comma_separated(&field, FIELDS_END)(s)?;

    let (s, close_brace_space) = take_whitespace0(s)?;
    let (s, close_brace) = tag(FIELDS_END)(s)?;

    let mut output = vec![
        dialect::HighlightedSpan {
            text: open_brace,
            group: Some(dialect::HighlightGroup::Delimiter),
        },
        dialect::HighlightedSpan {
            text: open_brace_space,
            group: None,
        },
    ];

    output.append(&mut fields);

    output.extend_from_slice(&[
        dialect::HighlightedSpan {
            text: close_brace_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: close_brace,
            group: Some(dialect::HighlightGroup::Delimiter),
        },
    ]);

    Ok((s, output))
}

fn field(s: &str) -> ParseResult<'_> {
    let (s, name) = snake_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, colon) = tag(":")(s)?;
    let (s, colon_space) = take_whitespace0(s)?;

    let (s, mut value) = crate::expr(s)?;

    let mut output = vec![
        dialect::HighlightedSpan {
            text: name,
            group: Some(dialect::HighlightGroup::MemberUse),
        },
        dialect::HighlightedSpan {
            text: name_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: colon,
            group: Some(dialect::HighlightGroup::Separator),
        },
        dialect::HighlightedSpan {
            text: colon_space,
            group: None,
        },
    ];

    output.append(&mut value);

    Ok((s, output))
}
