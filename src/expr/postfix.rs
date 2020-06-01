use {
    crate::{
        utils::{snake_case, take_whitespace0},
        ParseResult,
    },
    nom::{branch::alt, bytes::complete::tag, combinator::map},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    alt((method_call, field_access, try_oper))(s)
}

fn method_call(s: &str) -> ParseResult<'_> {
    let (s, period) = tag(".")(s)?;
    let (s, period_space) = take_whitespace0(s)?;

    let (s, mut function_call) = super::fn_call(s)?;

    let mut output = vec![
        dialect::HighlightedSpan {
            text: period,
            group: Some(dialect::HighlightGroup::MemberOper),
        },
        dialect::HighlightedSpan {
            text: period_space,
            group: None,
        },
    ];

    output.append(&mut function_call);

    Ok((s, output))
}

fn field_access(s: &str) -> ParseResult<'_> {
    let (s, period) = tag(".")(s)?;
    let (s, period_space) = take_whitespace0(s)?;

    let (s, field) = snake_case(s)?;

    let output = vec![
        dialect::HighlightedSpan {
            text: period,
            group: Some(dialect::HighlightGroup::MemberOper),
        },
        dialect::HighlightedSpan {
            text: period_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: field,
            group: Some(dialect::HighlightGroup::MemberUse),
        },
    ];

    Ok((s, output))
}

fn try_oper(s: &str) -> ParseResult<'_> {
    map(tag("?"), |s| {
        vec![dialect::HighlightedSpan {
            text: s,
            group: Some(dialect::HighlightGroup::OtherOper),
        }]
    })(s)
}
