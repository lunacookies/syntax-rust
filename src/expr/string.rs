use {
    crate::ParseResult,
    nom::bytes::complete::{tag, take_until},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, start_quote) = tag("\"")(s)?;
    let (s, contents) = take_until("\"")(s)?;
    let (s, end_quote) = tag("\"")(s)?;

    let output = vec![
        dialect::HighlightedSpan {
            text: start_quote,
            group: Some(dialect::HighlightGroup::StringDelimiter),
        },
        dialect::HighlightedSpan {
            text: contents,
            group: Some(dialect::HighlightGroup::String),
        },
        dialect::HighlightedSpan {
            text: end_quote,
            group: Some(dialect::HighlightGroup::StringDelimiter),
        },
    ];

    Ok((s, output))
}
