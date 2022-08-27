use std::str::Chars;

use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub(crate) struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self { chars: src.chars() }
    }

    fn read(&mut self) -> Option<char> {
        Some(self.chars.next()?)
    }
}

impl Lexer<'_> {
    pub(crate) fn next_token(&mut self) -> Token {
        if let Some(ch) = self.read() {
            match ch {
                '(' => Token::new(TokenKind::OpenParen, ch),
                ')' => Token::new(TokenKind::CloseParen, ch),
                _ => Token::new(TokenKind::Illegal, ch),
            }
        } else {
            Token::new(TokenKind::EOF, '\0')
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token_parens() {
        let input = "()";
        let expexted = vec![(TokenKind::OpenParen, '('), (TokenKind::CloseParen, ')')];

        let mut lexer = Lexer::new(input);
        for ex in expexted {
            let token = lexer.next_token();
            assert_eq!(token.kind, ex.0);
            assert_eq!(token.literal, ex.1);
        }
    }
}
