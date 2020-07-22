use crate::ParserOutput;

pub(crate) fn parse(_: &str) -> ParserOutput<'_> {
    Ok(("", vec![]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nothing() {
        assert_eq!(parse(""), Ok(("", vec![])));
    }
}
