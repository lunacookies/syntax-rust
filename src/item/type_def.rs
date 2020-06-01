use {
    crate::{
        utils::{pascal_case, take_whitespace0, take_whitespace1},
        ParseResult,
    },
    nom::{bytes::complete::tag, combinator::opt},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, keyword) = tag("type")(s)?;
    let (s, keyword_space) = take_whitespace1(s)?;

    let (s, name) = pascal_case(s)?;
    let (s, name_space) = take_whitespace0(s)?;

    let (s, rhs) = opt(|s| {
        let (s, equals) = tag("=")(s)?;
        let (s, equals_space) = take_whitespace0(s)?;

        let (s, mut ty) = crate::ty(s)?;

        let mut output = vec![
            dialect::HighlightedSpan {
                text: equals,
                group: Some(dialect::HighlightGroup::AssignOper),
            },
            dialect::HighlightedSpan {
                text: equals_space,
                group: None,
            },
        ];

        output.append(&mut ty);

        Ok((s, output))
    })(s)?;

    let (s, rhs_space) = take_whitespace0(s)?;

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
        dialect::HighlightedSpan {
            text: name,
            group: Some(dialect::HighlightGroup::TyDef),
        },
        dialect::HighlightedSpan {
            text: name_space,
            group: None,
        },
    ];

    if let Some(mut rhs) = rhs {
        output.append(&mut rhs);
    }

    output.extend_from_slice(&[
        dialect::HighlightedSpan {
            text: rhs_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: semicolon,
            group: Some(dialect::HighlightGroup::Terminator),
        },
    ]);

    Ok((s, output))
}
