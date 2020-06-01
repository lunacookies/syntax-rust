use {
    crate::{
        utils::{comma_separated, snake_case, take_whitespace0, take_whitespace1},
        ParseResult,
    },
    nom::{
        branch::alt,
        bytes::complete::tag,
        combinator::{map, opt},
    },
};

const PARAMS_START: &str = "(";
const PARAMS_END: &str = ")";

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("fn")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = snake_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, generics) = opt(crate::generics::def)(s)?;
    let (s, generics_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag(PARAMS_START)(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    let (s, mut params) = comma_separated(&param, PARAMS_END)(s)?;

    let (s, close_paren) = tag(PARAMS_END)(s)?;
    let (s, close_paren_space) = take_whitespace0(s)?;

    let (s, return_type) = opt(return_type)(s)?;
    let (s, return_type_space) = take_whitespace0(s)?;

    let semicolon = map(tag(";"), |s| {
        vec![dialect::HighlightedSpan {
            text: s,
            group: Some(dialect::HighlightGroup::Terminator),
        }]
    });

    // Function bodies can be either a block expression, or simply a semicolon (as in traits).
    let (s, mut body) = alt((crate::block, semicolon))(s)?;

    let mut output = vec![
        dialect::HighlightedSpan {
            text: keyword,
            group: Some(dialect::HighlightGroup::OtherKeyword),
        },
        dialect::HighlightedSpan {
            text: keyword_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: name,
            group: Some(dialect::HighlightGroup::FunctionDef),
        },
        dialect::HighlightedSpan {
            text: name_space,
            group: None,
        },
    ];

    if let Some(mut generics) = generics {
        output.append(&mut generics);
    }

    output.extend_from_slice(&[
        dialect::HighlightedSpan {
            text: generics_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: open_paren,
            group: Some(dialect::HighlightGroup::Delimiter),
        },
        dialect::HighlightedSpan {
            text: open_paren_space,
            group: None,
        },
    ]);

    output.append(&mut params);

    output.extend_from_slice(&[
        dialect::HighlightedSpan {
            text: close_paren,
            group: Some(dialect::HighlightGroup::Delimiter),
        },
        dialect::HighlightedSpan {
            text: close_paren_space,
            group: None,
        },
    ]);

    if let Some(mut return_type) = return_type {
        output.append(&mut return_type);
    }

    output.push(dialect::HighlightedSpan {
        text: return_type_space,
        group: None,
    });

    output.append(&mut body);

    Ok((s, output))
}

fn param(s: &str) -> ParseResult<'_> {
    let (s, name) = snake_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, colon) = tag(":")(s)?;
    let (s, colon_space) = take_whitespace0(s)?;

    let (s, mut ty) = crate::ty(s)?;

    let mut output = vec![
        dialect::HighlightedSpan {
            text: name,
            group: Some(dialect::HighlightGroup::FunctionParam),
        },
        dialect::HighlightedSpan {
            text: name_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: colon,
            group: Some(dialect::HighlightGroup::Separator),
        },
        dialect::HighlightedSpan {
            text: colon_space,
            group: None,
        },
    ];

    output.append(&mut ty);

    Ok((s, output))
}

fn return_type(s: &str) -> ParseResult<'_> {
    let (s, arrow) = tag("->")(s)?;
    let (s, arrow_space) = take_whitespace0(s)?;

    let (s, mut ty) = crate::ty(s)?;

    let mut output = vec![
        dialect::HighlightedSpan {
            text: arrow,
            group: Some(dialect::HighlightGroup::Separator),
        },
        dialect::HighlightedSpan {
            text: arrow_space,
            group: None,
        },
    ];

    output.append(&mut ty);

    Ok((s, output))
}
