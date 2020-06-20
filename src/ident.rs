use nom::bytes::complete::take_while1;

pub(crate) fn snake_case(s: &str) -> nom::IResult<&str, &str> {
    let _ = take_while1(|c: char| c.is_ascii_lowercase() || c == '_')(s)?;
    take_while1(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_snake_case_with_lowercase() {
        assert_eq!(snake_case("snake_case"), Ok(("", "snake_case")));
    }

    #[test]
    fn cannot_parse_snake_case_with_uppercase() {
        assert!(snake_case("CamelCase").is_err());
    }

    #[test]
    fn parse_snake_case_with_numbers() {
        assert_eq!(snake_case("foobar123"), Ok(("", "foobar123")));
    }

    #[test]
    fn cannot_parse_snake_case_starting_with_number() {
        assert!(snake_case("1foo").is_err());
    }

    #[test]
    fn cannot_parse_snake_case_starting_with_underscore() {
        assert_eq!(snake_case("_unused"), Ok(("", "_unused")));
    }
}
