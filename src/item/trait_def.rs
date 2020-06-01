use {
    crate::{
        utils::{pascal_case, take_whitespace0, take_whitespace1},
        ParseResult,
    },
    nom::{bytes::complete::tag, combinator::opt, multi::many0},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("trait")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = pascal_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, generics) = opt(crate::generics::def)(s)?;
    let (s, generics_space) = take_whitespace0(s)?;

    let (s, bounds) = opt(crate::bounds)(s)?;
    let (s, bounds_space) = take_whitespace0(s)?;

    let (s, open_brace) = tag("{")(s)?;
    let (s, open_brace_space) = take_whitespace0(s)?;

    let (s, items) = many0(|s| {
        let (s, item) = crate::item(s)?;
        let (s, space) = take_whitespace0(s)?;

        let mut output = item;
        output.push(dialect::HighlightedSpan {
            text: space,
            group: None,
        });

        Ok((s, output))
    })(s)?;

    let (s, close_brace_space) = take_whitespace0(s)?;
    let (s, close_brace) = tag("}")(s)?;

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
            group: Some(dialect::HighlightGroup::InterfaceDef),
        },
        dialect::HighlightedSpan {
            text: name_space,
            group: None,
        },
    ];

    if let Some(mut generics) = generics {
        output.append(&mut generics);
    }

    output.push(dialect::HighlightedSpan {
        text: generics_space,
        group: None,
    });

    if let Some(mut bounds) = bounds {
        output.append(&mut bounds);
    }

    output.extend_from_slice(&[
        dialect::HighlightedSpan {
            text: bounds_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: open_brace,
            group: Some(dialect::HighlightGroup::Delimiter),
        },
        dialect::HighlightedSpan {
            text: open_brace_space,
            group: None,
        },
    ]);

    output.append(&mut items.concat());

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
