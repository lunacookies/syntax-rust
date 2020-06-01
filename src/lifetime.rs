use {
    crate::{utils::snake_case, ParseResult},
    nom::{bytes::complete::tag, combinator::map, sequence::pair},
};

pub(crate) fn usage(s: &str) -> ParseResult<'_> {
    map(pair(tag("'"), snake_case), |(tick, name)| {
        vec![
            dialect::HighlightedSpan {
                text: tick,
                group: Some(dialect::HighlightGroup::SpecialIdentUse),
            },
            dialect::HighlightedSpan {
                text: name,
                group: Some(dialect::HighlightGroup::SpecialIdentUse),
            },
        ]
    })(s)
}

pub(crate) fn def(s: &str) -> ParseResult<'_> {
    map(pair(tag("'"), snake_case), |(tick, name)| {
        vec![
            dialect::HighlightedSpan {
                text: tick,
                group: Some(dialect::HighlightGroup::SpecialIdentDef),
            },
            dialect::HighlightedSpan {
                text: name,
                group: Some(dialect::HighlightGroup::SpecialIdentDef),
            },
        ]
    })(s)
}
