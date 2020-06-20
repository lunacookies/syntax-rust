use nom::bytes::complete::take_while1;

pub(crate) fn camel_case(s: &str) -> nom::IResult<&str, &str> {
    let _ = take_while1(|c: char| c.is_ascii_uppercase())(s)?;
    take_while1(|c: char| c.is_ascii_alphanumeric())(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_camel_case_with_all_caps() {
        assert_eq!(camel_case("ALLCAPS"), Ok(("", "ALLCAPS")));
    }

    #[test]
    fn parse_camel_case_with_mixed_case() {
        assert_eq!(camel_case("CamelCase"), Ok(("", "CamelCase")));
    }

    #[test]
    fn parse_camel_case_with_numbers() {
        assert_eq!(camel_case("Camel123Case456"), Ok(("", "Camel123Case456")));
    }

    #[test]
    fn cannot_parse_camel_case_starting_with_number() {
        assert!(camel_case("999WontWork").is_err());
    }

    #[test]
    fn cannot_parse_camel_case_starting_with_lowercase() {
        assert!(camel_case("bad").is_err());
    }

    #[test]
    fn cannot_parse_camel_case_containing_underscore() {
        use nom::combinator::all_consuming;
        assert!(all_consuming(camel_case)("Not_Camel_Case").is_err());
    }

    // TODO: make this work.
    //
    // #[test]
    // fn parse_camel_case_starting_with_underscore() {
    //     assert_eq!(camel_case("_Unused"), Ok(("", "_Unused")));
    // }
}
