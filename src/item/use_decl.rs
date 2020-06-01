use {
    crate::{
        utils::{take_whitespace0, take_whitespace1},
        ParseResult,
    },
    nom::{branch::alt, bytes::complete::tag},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("use")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, mut path) = crate::path(s)?;
    let (s, path_space) = take_whitespace0(s)?;

    let (s, mut ident) = alt((crate::ty_name, crate::module_name))(s)?;
    let (s, ident_space) = take_whitespace0(s)?;

    let (s, semicolon) = tag(";")(s)?;

    let mut output = vec![
        dialect::HighlightedSpan {
            text: keyword,
            group: Some(dialect::HighlightGroup::OtherKeyword),
        },
        dialect::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
    ];

    output.append(&mut path);

    output.push(dialect::HighlightedSpan {
        text: path_space,
        group: None,
    });

    output.append(&mut ident);

    output.extend_from_slice(&[
        dialect::HighlightedSpan {
            text: ident_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: semicolon,
            group: Some(dialect::HighlightGroup::Terminator),
        },
    ]);

    Ok((s, output))
}
