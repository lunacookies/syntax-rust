use {
    crate::{utils::take_whitespace0, ParseResult},
    nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::pair},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    alt((deref, borrow_mut, borrow))(s)
}

fn deref(s: &str) -> ParseResult<'_> {
    map(pair(tag("*"), take_whitespace0), |(oper, space)| {
        vec![
            dialect::HighlightedSpan {
                text: oper,
                group: Some(dialect::HighlightGroup::PointerOper),
            },
            dialect::HighlightedSpan {
                text: space,
                group: None,
            },
        ]
    })(s)
}

fn borrow_mut(s: &str) -> ParseResult<'_> {
    map(pair(tag("&mut"), take_whitespace0), |(oper, space)| {
        vec![
            dialect::HighlightedSpan {
                text: oper,
                group: Some(dialect::HighlightGroup::PointerOper),
            },
            dialect::HighlightedSpan {
                text: space,
                group: None,
            },
        ]
    })(s)
}

fn borrow(s: &str) -> ParseResult<'_> {
    map(pair(tag("&"), take_whitespace0), |(oper, space)| {
        vec![
            dialect::HighlightedSpan {
                text: oper,
                group: Some(dialect::HighlightGroup::PointerOper),
            },
            dialect::HighlightedSpan {
                text: space,
                group: None,
            },
        ]
    })(s)
}
