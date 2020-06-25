use nom::bytes::complete::take_while1;

pub(crate) fn binary(s: &str) -> nom::IResult<&str, &str> {
    take_while1(|c| c == '0' || c == '1')(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_zeros() {
        assert_eq!(binary("000"), Ok(("", "000")));
    }

    #[test]
    fn parse_ones() {
        assert_eq!(binary("11"), Ok(("", "11")));
    }

    #[test]
    fn parse_ones_and_zeros() {
        assert_eq!(
            binary("11010001100101110110011011001101111"),
            Ok(("", "11010001100101110110011011001101111"))
        );
    }
}
