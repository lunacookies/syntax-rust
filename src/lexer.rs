use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
enum Token {
    #[regex("_?[A-Z][A-Za-z0-9]*")]
    Type,
    #[error]
    #[regex("[ \t\n\r]", logos::skip)]
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

    #[test]
    fn skips_line_feeds() {
        assert_eq!(Token::lexer("\n\n\n\n").count(), 0);
    }

    #[test]
    fn skips_carriage_returns() {
        assert_eq!(Token::lexer("\r\r").count(), 0);
    }

    #[test]
    fn types_are_pascal_case() {
        let mut lexer = Token::lexer("PascalCase123");
        assert_eq!(lexer.next(), Some(Token::Type));
        assert_eq!(lexer.slice(), "PascalCase123");
    }

    #[test]
    fn types_cannot_start_with_number() {
        assert_ne!(Token::lexer("123NotAType").next(), Some(Token::Type));
    }

    #[test]
    fn types_can_start_with_an_underscore() {
        let mut lexer = Token::lexer("_Unused123Type");
        assert_eq!(lexer.next(), Some(Token::Type));
        assert_eq!(lexer.slice(), "_Unused123Type");
    }
}
