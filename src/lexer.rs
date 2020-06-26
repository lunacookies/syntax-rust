use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
enum Token {
    #[error]
    #[regex("[ \t]", logos::skip)]
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

    #[test]
    fn skips_tabs() {
        assert_eq!(Token::lexer("\t\t\t").count(), 0);
    }
}
