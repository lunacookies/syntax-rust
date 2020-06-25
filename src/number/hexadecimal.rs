use nom::bytes::complete::take_while1;

pub(crate) fn hexadecimal(s: &str) -> nom::IResult<&str, &str> {
    take_while1(|c: char| {
        c.is_ascii_digit() || ('a'..='f').contains(&c) || ('A'..='F').contains(&c)
    })(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_digits_between_0_and_9() {
        assert_eq!(hexadecimal("1234567890"), Ok(("", "1234567890")));
    }

    #[test]
    fn parse_digits_between_a_and_f() {
        assert_eq!(hexadecimal("abcdef"), Ok(("", "abcdef")));
    }

    #[test]
    #[allow(non_snake_case)]
    fn parse_digits_between_A_and_F() {
        assert_eq!(hexadecimal("ABCDEF"), Ok(("", "ABCDEF")));
    }
}
