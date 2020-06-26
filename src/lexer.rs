use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
enum Token {
    #[error]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_nothing() {
        assert_eq!(Token::lexer("").collect::<Vec<_>>(), Vec::new());
    }
}
