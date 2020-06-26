use logos::Logos;

#[derive(Debug, PartialEq, Logos)]
enum Token {
    #[regex("_?[A-Z][A-Za-z0-9]*")]
    Type,
    #[regex("_?[a-z][a-z0-9_]*")]
    Ident,
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

    #[test]
    fn idents_are_snake_case() {
        let mut lexer = Token::lexer("snake_123_case");
        assert_eq!(lexer.next(), Some(Token::Ident));
        assert_eq!(lexer.slice(), "snake_123_case");
    }

    #[test]
    fn idents_cannot_start_with_number() {
        assert_ne!(Token::lexer("123ident").next(), Some(Token::Ident));
    }

    #[test]
    fn idents_can_start_with_an_underscore() {
        let mut lexer = Token::lexer("_unused_ident");
        assert_eq!(lexer.next(), Some(Token::Ident));
        assert_eq!(lexer.slice(), "_unused_ident");
    }
}
