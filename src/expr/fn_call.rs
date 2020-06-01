use {
    crate::{
        utils::{comma_separated, snake_case, take_whitespace0},
        ParseResult,
    },
    nom::{bytes::complete::tag, combinator::opt},
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    let (s, path) = crate::path(s)?;
    let (s, name) = snake_case(s)?;

    let (s, turbofish) = opt(|s| {
        let (s, double_colon) = tag("::")(s)?;
        let (s, mut generics) = crate::generics::usage(s)?;

        let mut output = vec![dialect::HighlightedSpan {
            text: double_colon,
            group: Some(dialect::HighlightGroup::Separator),
        }];

        output.append(&mut generics);

        Ok((s, output))
    })(s)?;

    let (s, turbofish_space) = take_whitespace0(s)?;

    let (s, open_paren) = tag("(")(s)?;
    let (s, open_paren_space) = take_whitespace0(s)?;

    // Function calls take in expressions.
    let (s, mut params) = comma_separated(&crate::expr, ")")(s)?;
    let (s, params_space) = take_whitespace0(s)?;

    let (s, close_paren) = tag(")")(s)?;

    let mut output = path;

    output.push(dialect::HighlightedSpan {
        text: name,
        group: Some(dialect::HighlightGroup::FunctionCall),
    });

    if let Some(mut turbofish) = turbofish {
        output.append(&mut turbofish);
    }

    output.extend_from_slice(&[
        dialect::HighlightedSpan {
            text: turbofish_space,
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
            text: params_space,
            group: None,
        },
        dialect::HighlightedSpan {
            text: close_paren,
            group: Some(dialect::HighlightGroup::Delimiter),
        },
    ]);

    Ok((s, output))
}
