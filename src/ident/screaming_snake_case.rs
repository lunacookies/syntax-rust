use nom::bytes::complete::take_while1;

pub(crate) fn screaming_snake_case(s: &str) -> nom::IResult<&str, &str> {
    let _ = take_while1(|c: char| c.is_ascii_uppercase() || c == '_')(s)?;
    take_while1(|c: char| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_screaming_snake_case_without_underscores() {
        assert_eq!(screaming_snake_case("SCREAMING"), Ok(("", "SCREAMING")));
    }

    #[test]
    fn parse_screaming_snake_case_with_underscores() {
        assert_eq!(screaming_snake_case("SNAKE_CASE"), Ok(("", "SNAKE_CASE")));
    }

    #[test]
    fn parse_screaming_snake_case_with_numbers() {
        assert_eq!(screaming_snake_case("WITH1234"), Ok(("", "WITH1234")));
    }

    #[test]
    fn cannot_parse_screaming_snake_case_starting_with_number() {
        assert!(screaming_snake_case("123_WONT_WORK").is_err());
    }

    #[test]
    fn parse_screaming_snake_case_starting_with_underscore() {
        assert_eq!(screaming_snake_case("_UNUSED"), Ok(("", "_UNUSED")));
    }
}
