use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
enum Token {
    #[error]
    #[token(" ", logos::skip)]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_nothing() {
        assert_eq!(Token::lexer("").count(), 0);
    }

    #[test]
    fn skips_spaces() {
        assert_eq!(Token::lexer("  ").count(), 0);
    }
}
