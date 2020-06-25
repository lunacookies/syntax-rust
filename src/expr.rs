use crate::{snake_case, ParserOutput};
use dialect::{HighlightGroup, HighlightedSpan};
use nom::combinator::map;

pub(crate) fn expr(s: &str) -> ParserOutput<'_> {
    var(s)
}

fn var(s: &str) -> ParserOutput<'_> {
    map(snake_case, |s| {
        vec![HighlightedSpan {
            text: s,
            group: Some(HighlightGroup::VariableUse),
        }]
    })(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_variable() {
        assert_eq!(
            expr("foobar"),
            Ok((
                "",
                vec![HighlightedSpan {
                    text: "foobar",
                    group: Some(HighlightGroup::VariableUse)
                }]
            ))
        );
    }
}
