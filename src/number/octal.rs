use nom::bytes::complete::take_while1;

pub(crate) fn octal(s: &str) -> nom::IResult<&str, &str> {
    take_while1(|c| c >= '0' && c < '8')(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_digits_between_0_and_8() {
        assert_eq!(octal("01234567"), Ok(("", "01234567")));
    }

    #[test]
    fn cannot_parse_digits_above_and_including_8() {
        assert!(octal("89").is_err());
    }
}
