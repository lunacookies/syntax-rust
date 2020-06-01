use {
    crate::ParseResult,
    nom::bytes::complete::{tag, take},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, start_quote) = tag("'")(s)?;
    let (s, contents) = take(1usize)(s)?;
    let (s, end_quote) = tag("'")(s)?;

    let output = vec![
        dialect::HighlightedSpan {
            text: start_quote,
            group: Some(dialect::HighlightGroup::CharacterDelimiter),
        },
        dialect::HighlightedSpan {
            text: contents,
            group: Some(dialect::HighlightGroup::Character),
        },
        dialect::HighlightedSpan {
            text: end_quote,
            group: Some(dialect::HighlightGroup::CharacterDelimiter),
        },
    ];

    Ok((s, output))
}
