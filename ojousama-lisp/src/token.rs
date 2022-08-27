#[derive(Debug, PartialEq)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum TokenKind {
    Illegal,
    EOF,
    Identfier,
    // Int,
    // Comma,
    OpenParen,
    CloseParen,
}
