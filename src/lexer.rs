use logos::Logos;

pub(crate) fn lex(s: &str) -> Vec<Token> {
    let mut tokens: Vec<_> = TokenKind::lexer(s)
        .spanned()
        .map(|(kind, range)| Token { kind, range })
        .collect();

    tokens.reverse();

    tokens
}

#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) range: std::ops::Range<usize>,
}

#[derive(Debug, PartialEq, Logos)]
pub(crate) enum TokenKind {
    #[token("fn")]
    Fn,
    #[token("struct")]
    Struct,
    #[token("trait")]
    Trait,
    #[token("let")]
    Let,
    #[regex("_?[A-Z][A-Za-z0-9]*")]
    TypeIdent,
    #[regex("_?[a-z][a-z0-9_]*")]
    Ident,
    #[regex("'_?[a-z][a-z0-9_]*")]
    TickIdent,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("=")]
    Equals,
    #[token(";")]
    Semi,
    #[token(",")]
    Comma,
    #[token("::")]
    DoubleColon,
    #[token("->")]
    ThinArrow,
    #[error]
    #[regex("[ \t\n\r]", logos::skip)]
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn lexes_nothing() {
        assert_eq!(TokenKind::lexer("").count(), 0);
    }

    #[test]
    fn skips_spaces() {
        assert_eq!(TokenKind::lexer("  ").count(), 0);
    }

    #[test]
    fn skips_tabs() {
        assert_eq!(TokenKind::lexer("\t\t\t").count(), 0);
    }

    #[test]
    fn skips_line_feeds() {
        assert_eq!(TokenKind::lexer("\n\n\n\n").count(), 0);
    }

    #[test]
    fn skips_carriage_returns() {
        assert_eq!(TokenKind::lexer("\r\r").count(), 0);
    }

    #[test]
    fn lexes_fn() {
        let mut lexer = TokenKind::lexer("fn");
        assert_eq!(lexer.next(), Some(TokenKind::Fn));
        assert_eq!(lexer.slice(), "fn");
    }

    #[test]
    fn lexes_struct() {
        let mut lexer = TokenKind::lexer("struct");
        assert_eq!(lexer.next(), Some(TokenKind::Struct));
        assert_eq!(lexer.slice(), "struct");
    }

    #[test]
    fn lexes_trait() {
        let mut lexer = TokenKind::lexer("trait");
        assert_eq!(lexer.next(), Some(TokenKind::Trait));
        assert_eq!(lexer.slice(), "trait");
    }

    #[test]
    fn lexes_let() {
        let mut lexer = TokenKind::lexer("let");
        assert_eq!(lexer.next(), Some(TokenKind::Let));
        assert_eq!(lexer.slice(), "let");
    }

    #[test]
    fn types_are_pascal_case() {
        let mut lexer = TokenKind::lexer("PascalCase123");
        assert_eq!(lexer.next(), Some(TokenKind::TypeIdent));
        assert_eq!(lexer.slice(), "PascalCase123");
    }

    #[test]
    fn types_cannot_start_with_number() {
        assert_ne!(
            TokenKind::lexer("123NotAType").next(),
            Some(TokenKind::TypeIdent)
        );
    }

    #[test]
    fn types_can_start_with_an_underscore() {
        let mut lexer = TokenKind::lexer("_Unused123Type");
        assert_eq!(lexer.next(), Some(TokenKind::TypeIdent));
        assert_eq!(lexer.slice(), "_Unused123Type");
    }

    #[test]
    fn idents_are_snake_case() {
        let mut lexer = TokenKind::lexer("snake_123_case");
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.slice(), "snake_123_case");
    }

    #[test]
    fn idents_cannot_start_with_number() {
        assert_ne!(TokenKind::lexer("123ident").next(), Some(TokenKind::Ident));
    }

    #[test]
    fn idents_can_start_with_an_underscore() {
        let mut lexer = TokenKind::lexer("_unused_ident");
        assert_eq!(lexer.next(), Some(TokenKind::Ident));
        assert_eq!(lexer.slice(), "_unused_ident");
    }

    #[test]
    fn lifetimes_are_snake_case_with_quote() {
        let mut lexer = TokenKind::lexer("'snake_case");
        assert_eq!(lexer.next(), Some(TokenKind::TickIdent));
        assert_eq!(lexer.slice(), "'snake_case");
    }

    #[test]
    fn lifetimes_cannot_start_with_number() {
        assert_ne!(
            TokenKind::lexer("'123lifetime").next(),
            Some(TokenKind::TickIdent)
        );
    }

    #[test]
    fn lifetimes_can_start_with_an_underscore() {
        let mut lexer = TokenKind::lexer("'_unused_lifetime");
        assert_eq!(lexer.next(), Some(TokenKind::TickIdent));
        assert_eq!(lexer.slice(), "'_unused_lifetime");
    }

    #[test]
    fn lexes_open_paren() {
        let mut lexer = TokenKind::lexer("(");
        assert_eq!(lexer.next(), Some(TokenKind::OpenParen));
        assert_eq!(lexer.slice(), "(");
    }

    #[test]
    fn lexes_close_paren() {
        let mut lexer = TokenKind::lexer(")");
        assert_eq!(lexer.next(), Some(TokenKind::CloseParen));
        assert_eq!(lexer.slice(), ")");
    }

    #[test]
    fn lexes_open_brace() {
        let mut lexer = TokenKind::lexer("{");
        assert_eq!(lexer.next(), Some(TokenKind::OpenBrace));
        assert_eq!(lexer.slice(), "{");
    }

    #[test]
    fn lexes_close_brace() {
        let mut lexer = TokenKind::lexer("}");
        assert_eq!(lexer.next(), Some(TokenKind::CloseBrace));
        assert_eq!(lexer.slice(), "}");
    }

    #[test]
    fn lexes_equals() {
        let mut lexer = TokenKind::lexer("=");
        assert_eq!(lexer.next(), Some(TokenKind::Equals));
        assert_eq!(lexer.slice(), "=");
    }

    #[test]
    fn lexes_semicolon() {
        let mut lexer = TokenKind::lexer(";");
        assert_eq!(lexer.next(), Some(TokenKind::Semi));
        assert_eq!(lexer.slice(), ";");
    }

    #[test]
    fn lexes_comma() {
        let mut lexer = TokenKind::lexer(",");
        assert_eq!(lexer.next(), Some(TokenKind::Comma));
        assert_eq!(lexer.slice(), ",");
    }

    #[test]
    fn lexes_double_colon() {
        let mut lexer = TokenKind::lexer("::");
        assert_eq!(lexer.next(), Some(TokenKind::DoubleColon));
        assert_eq!(lexer.slice(), "::");
    }

    #[test]
    fn lexes_thin_arrow() {
        let mut lexer = TokenKind::lexer("->");
        assert_eq!(lexer.next(), Some(TokenKind::ThinArrow));
        assert_eq!(lexer.slice(), "->");
    }
}
