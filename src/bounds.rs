use {
    crate::{utils::take_whitespace0, ParseResult},
    nom::{branch::alt, bytes::complete::tag, multi::many0},
};

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    let (s, colon) = tag(":")(s)?;
    let (s, colon_space) = take_whitespace0(s)?;

    let (s, mut first) = bound(s)?;

    let (s, rest) = many0(|s| {
        let (s, space) = take_whitespace0(s)?;

        let (s, plus) = tag("+")(s)?;
        let (s, plus_space) = take_whitespace0(s)?;

        let (s, mut bound) = bound(s)?;

        let mut output = vec![
            dialect::HighlightedSpan {
                text: space,
                group: None,
            },
            dialect::HighlightedSpan {
                text: plus,
                group: Some(dialect::HighlightGroup::BinaryOper),
            },
            dialect::HighlightedSpan {
                text: plus_space,
                group: None,
            },
        ];

        output.append(&mut bound);

        Ok((s, output))
    })(s)?;

    let mut output = vec![
        dialect::HighlightedSpan {
            text: colon,
            group: Some(dialect::HighlightGroup::Separator),
        },
        dialect::HighlightedSpan {
            text: colon_space,
            group: None,
        },
    ];

    output.append(&mut first);
    output.append(&mut rest.concat());

    Ok((s, output))
}

fn bound(s: &str) -> ParseResult<'_> {
    alt((crate::lifetime::usage, crate::trait_))(s)
}
