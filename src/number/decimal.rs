use nom::bytes::complete::take_while1;

pub(crate) fn decimal(s: &str) -> nom::IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_digit())(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_digits() {
        assert_eq!(decimal("1234567890"), Ok(("", "1234567890")));
    }
}
