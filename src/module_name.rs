use {
    crate::{utils::snake_case, ParseResult},
    nom::combinator::map,
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    map(snake_case, |s| {
        vec![dialect::HighlightedSpan {
            text: s,
            group: Some(dialect::HighlightGroup::ModuleUse),
        }]
    })(s)
}
