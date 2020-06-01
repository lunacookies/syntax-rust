use {
    crate::{
        utils::{expect, take_whitespace0},
        ParseResult,
    },
    nom::{bytes::complete::tag, multi::many0},
};

const BLOCK_START: &str = "{";
const BLOCK_END: &str = "}";

pub(crate) fn parse(s: &str) -> ParseResult<'_> {
    let (s, open_brace) = tag(BLOCK_START)(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    let (s, statements) = many0(|s| {
        let (s, statement) = expect(crate::statement, Some(BLOCK_END))(s)?;
        let (s, space) = take_whitespace0(s)?;

        let mut output = statement;
        output.push(dialect::HighlightedSpan {
            text: space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

    let (s, close_brace_space) = take_whitespace0(s)?;
    let (s, close_brace) = tag(BLOCK_END)(s)?;

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

    output.append(&mut statements.concat());

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
