use {
    crate::{utils::snake_case, ParseResult},
    nom::combinator::map,
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    map(snake_case, |s| {
        vec![dialect::HighlightedSpan {
            text: s,
            group: Some(dialect::HighlightGroup::VariableUse),
        }]
    })(s)
}
